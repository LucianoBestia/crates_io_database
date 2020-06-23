// qvs20_reader_mod

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Qvs20ErrorReader {
    // reader
    #[error("Error: The field must start with [. pos: {pos}")]
    NoFieldStart { pos: usize },
    #[error("Error: Last bracket is missing. pos: {pos}")]
    NoFieldEnd { pos: usize },
    #[error("Error: Last row delimiter is missing. pos: {pos}")]
    NoLastRowDelimiter { pos: usize },
    #[error("Error: The row delimiter has more than 1 byte. pos: {pos}")]
    RowDelimiterMoreThan1Byte { pos: usize },
    #[error("Error: Premature end of file. pos: {pos}")]
    PrematureEndOfFile { pos: usize },
}

/// ReaderForQvs20
pub struct ReaderForQvs20<'a> {
    // All the fields are internal and not public.
    // The only way to interact is through methods.
    /// reference to the string (no allocation), but as [u8] for performance
    input: &'a [u8],
    /// where is the cursor now
    cursor_state: CursorState,
    /// cursor position
    cursor_pos: usize,
}

/// internal enum
enum CursorState {
    /// start of file
    StartOfField,
    /// row_delimiter
    InsideRowDelimiter,
    /// inside of field  
    InsideOfField,
    /// outside of field
    OutsideOfField,
    /// reached normal end of file
    EndOfFile,
}

/// the returned Token from the iterator
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token<'a> {
    /// field content - not unescaped  
    Field(&'a [u8]),
    /// row delimiter  
    RowDelimiter(u8),
}

impl<'a> ReaderForQvs20<'a> {
    /// Constructor. String (but in [u8] form) is immutably borrowed here. No allocation.  
    pub fn new(input: &[u8]) -> ReaderForQvs20 {
        ReaderForQvs20 {
            input,
            cursor_state: CursorState::StartOfField,
            cursor_pos: 0,
        }
    }
    /// low level - find u8 from pos_cursor
    pub fn find_u8_from(source_str: &[u8], pos_cursor: usize, find_u8: u8) -> Option<usize> {
        //print!("find_u8_from {}, {}, {}",unwrap!(String::from_utf8(source_str.to_vec())),pos_cursor,unwrap!(String::from_utf8(vec![find_u8])));
        let slice01 = &source_str[pos_cursor..];
        let opt_pos_found = slice01.iter().position(|&s| s == find_u8);
        if let Some(pos_found) = opt_pos_found {
            // return Option with usize
            //println!("return {} {}",pos_cursor,pos_found);
            Some(pos_cursor + pos_found)
        } else {
            //println!("return None");
            // return
            None
        }
    }
}

impl<'a> Iterator for ReaderForQvs20<'a> {
    type Item = Result<Token<'a>, Qvs20ErrorReader>;
    /// Reads the next token. Return None when EndOfFile. Can return Error.
    fn next(&mut self) -> Option<Result<Token<'a>, Qvs20ErrorReader>> {
        if self.input.is_empty() {
            return None;
        }
        // This loop breaks only with return
        loop {
            match &self.cursor_state {
                CursorState::StartOfField => {
                    if self.input[self.cursor_pos] == b'[' {
                        self.cursor_state = CursorState::InsideOfField;
                        self.cursor_pos += 1;
                        if self.cursor_pos >= self.input.len() {
                            return Some(Err(Qvs20ErrorReader::PrematureEndOfFile {
                                pos: self.cursor_pos,
                            }));
                        }
                    // continue loop
                    } else {
                        return Some(Err(Qvs20ErrorReader::NoFieldStart {
                            pos: self.cursor_pos,
                        }));
                    }
                }
                CursorState::InsideOfField => {
                    let start_pos = self.cursor_pos;
                    while let Some(pos) = Self::find_u8_from(self.input, self.cursor_pos, b']') {
                        if self.input[pos - 1] == b'\\' {
                            // if before the delimiter is \ (escaped), then find the next
                            self.cursor_pos += 1;
                            if self.cursor_pos >= self.input.len() {
                                return Some(Err(Qvs20ErrorReader::PrematureEndOfFile {
                                    pos: self.cursor_pos,
                                }));
                            }
                        //continue while
                        } else {
                            self.cursor_pos = pos;
                            break;
                        }
                    }
                    if self.input[self.cursor_pos] != b']' {
                        return Some(Err(Qvs20ErrorReader::NoFieldEnd {
                            pos: self.cursor_pos,
                        }));
                    }
                    let end_pos = self.cursor_pos;
                    self.cursor_pos += 1;
                    self.cursor_state = CursorState::OutsideOfField;
                    return Some(Ok(Token::Field(&self.input[start_pos..end_pos])));
                }
                CursorState::OutsideOfField => {
                    if self.cursor_pos >= self.input.len() {
                        return Some(Err(Qvs20ErrorReader::NoLastRowDelimiter {
                            pos: self.cursor_pos,
                        }));
                    } else if self.input[self.cursor_pos] == b'[' {
                        self.cursor_state = CursorState::StartOfField;
                    } else {
                        self.cursor_state = CursorState::InsideRowDelimiter;
                    }
                }
                CursorState::InsideRowDelimiter => {
                    // row_delimiter must be only one byte 1,2,3..a,b,c..
                    // the hierarchy will never be very deep. Probably till 3.
                    if self.cursor_pos + 1 >= self.input.len() {
                        self.cursor_state = CursorState::EndOfFile;
                        return Some(Ok(Token::RowDelimiter(self.input[self.cursor_pos])));
                    } else if self.input[self.cursor_pos + 1] == b'[' {
                        self.cursor_state = CursorState::StartOfField;
                        let start_pos = self.cursor_pos;
                        self.cursor_pos += 1;
                        return Some(Ok(Token::RowDelimiter(self.input[start_pos])));
                    } else {
                        return Some(Err(Qvs20ErrorReader::RowDelimiterMoreThan1Byte {
                            pos: self.cursor_pos,
                        }));
                    }
                }
                CursorState::EndOfFile => {
                    // stop the iter() with None
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use unwrap::unwrap;
    #[test]
    pub fn test_01_well_formed() {
        // fields, escaped, row_delimiter, eof
        let mut rdr = ReaderForQvs20::new(
            r"[one][two][1\\2\]3\[4\n5\r6\t]
[four]
".as_bytes(),
        );
        // first field
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::Field("one".as_bytes()));
        // second field
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::Field("two".as_bytes()));
        // third field
        let token = unwrap!(unwrap!(rdr.next()));
        // here is raw bytes, not unescaped
        assert_eq!(token, Token::Field(r"1\\2\]3\[4\n5\r6\t".as_bytes()));
        // row_delimiter only one byte
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::RowDelimiter(b'\n'));
        // fourth field
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::Field(r"four".as_bytes()));
        // row_delimiter only one byte. Must end with row delimiter
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::RowDelimiter(b'\n'));
        // None is returned to signal the end for iter()
        let next = rdr.next();
        assert!(next.is_none());
    }
    
    #[test]
    pub fn test_02_not_field() {
        let mut rdr = ReaderForQvs20::new("this is not a field".as_bytes());
        // first field
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            Qvs20ErrorReader::NoFieldStart { pos: 0 }.to_string()
        );
    }
    
    #[test]
    pub fn test_03_empty() {
        let mut rdr = ReaderForQvs20::new("".as_bytes());
        // first field
        let opt_result = rdr.next();
        assert!(opt_result.is_none());
    }
    
    #[test]
    pub fn test_04_premature() {
        let mut rdr = ReaderForQvs20::new("[".as_bytes());
        // first field
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Error: Premature end of file. pos: 1");
    }

    #[test]
    pub fn test_05_no_last_bracket() {
        let mut rdr = ReaderForQvs20::new("[no last bracket".as_bytes());
        // first field
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            Qvs20ErrorReader::NoFieldEnd { pos: 1 }.to_string()
        );
    }
    
    #[test]
    pub fn test_06_no_last_bracket() {
        let mut rdr = ReaderForQvs20::new("[one][no last bracket".as_bytes());
        // first field
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::Field("one".as_bytes()));
        // second field
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            Qvs20ErrorReader::NoFieldEnd { pos: 6 }.to_string()
        );
    }
    #[test]
    pub fn test_07_premature_end() {
        let mut rdr = ReaderForQvs20::new("[escaped \\]".as_bytes());
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Error: Premature end of file. pos: 11"
        );
    }
    
    #[test]
    pub fn test_08_no_row_delimiter() {
        let mut rdr = ReaderForQvs20::new("[one]".as_bytes());
        // first field
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::Field("one".as_bytes()));
        // second field
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            Qvs20ErrorReader::NoLastRowDelimiter { pos: 5 }.to_string()
        );
    }
    
    #[test]
    pub fn test_09_row_delimiter_too_big() {
        let mut rdr = ReaderForQvs20::new("[one]\n\n".as_bytes());
        // first field
        let token = unwrap!(unwrap!(rdr.next()));
        assert_eq!(token, Token::Field("one".as_bytes()));
        // second field
        let result = unwrap!(rdr.next());
        assert_eq!(
            result.err().unwrap().to_string(),
            Qvs20ErrorReader::RowDelimiterMoreThan1Byte { pos: 5 }.to_string()
        );
    }
    
}
