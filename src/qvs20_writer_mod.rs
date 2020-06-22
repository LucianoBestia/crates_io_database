// qvs20_writer_mod

// TODO: write to bytes instead of String should be faster
// because there is no checking that the bytes are well-formed utf8

//use unwrap::unwrap;

/*
pub struct Writer {
    s: String,
    schema: qvs20_schema_mod::Schema,
    cursor_pos: usize,
    row_delimiter: char,
}

impl Writer {
    // constructor
    pub fn new(schema: qvs20_schema_mod::Schema) -> Self {
        //return
        Writer {
            s: String::with_capacity(1000),
            schema,
            cursor_pos: 0,
            row_delimiter: '\n',
        }
    }
    /// push a field of type String
    pub fn push_string(&mut self, data: &str) {
        assert!(self.schema.data_types[self.cursor_pos] == qvs20_schema_mod::DataType::String);
        self.s.push('[');
        let bytes = data.as_bytes();
        // TODO: this is bad place for allocation, it should be in the constructor.
        let mut escaped_buffer: Vec<u8> = vec![];
        self.s.push_str(&unwrap!(String::from_utf8(
            Self::escape_qvs20_str(bytes, &mut escaped_buffer).to_vec()
        )));
        self.s.push(']');
        self.cursor_pos += 1;
        if self.cursor_pos >= self.schema.column_names.len() {
            self.cursor_pos = 0;
            self.s.push(self.row_delimiter);
        }
    }

    /// escape the 6 special characters
    /// all this characters are ascii7
    /// therefore I can use a faster vector of bytes and not a string
    /// less escaping needed, faster the performance
    /// the parameter escaped is allocated before this fn call
    pub fn escape_qvs20_str<'a>(text: &'a [u8], escaped_buffer: &'a mut Vec<u8>) -> &'a [u8] {
        // \\, \[, \], \n, \r, \t
        // empty buffer, but retain allocation
        escaped_buffer.truncate(0);
        let mut inserted = 0;
        // not characters, but bytes !
        for (i, item) in text.iter().enumerate() {
            if item == &b'\\'
                || item == &b'['
                || item == &b']'
                || item == &b'\n'
                || item == &b'\t'
                || item == &b'\r'
            {
                if inserted == 0 {
                    //lazy, only if needed
                    escaped_buffer.extend_from_slice(&text.to_vec());
                }
                // for \t \n \r must replace this byte with a different byte
                if item == &b'\n' {
                    escaped_buffer[i + inserted] = b'n';
                } else if item == &b'\t' {
                    escaped_buffer[i + inserted] = b't';
                } else if item == &b'\r' {
                    escaped_buffer[i + inserted] = b'r';
                }
                //escaped becomes larger than text
                escaped_buffer.insert(i + inserted, b'\\');
                // one additional byte
                inserted += 1;
            }
        }
        //return
        if inserted > 0 {
            //println!("escape is_inserted: {}", unwrap!(str::from_utf8(escaped)));
            &escaped_buffer[..]
        } else {
            text
        }
    }
    pub fn bytes_for_file(&self) -> &[u8] {
        //return
        self.s.as_bytes()
    }
}
*/
