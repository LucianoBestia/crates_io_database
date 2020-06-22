// qvs20_table_mod

// The table structure is very flexible, because it is defined in runtime.
// A lot of times the table is used just as an intermediary,
// and don't need a fixed Rust struct in compile time.

use crate::qvs20_reader_mod::*;

//use strum;
use strum_macros::EnumString;
use std::str::FromStr;
use unwrap::unwrap;

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Bytes(Vec<u8>),
}

impl Default for Value {
    fn default() -> Self {
        Value::String("".to_string())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Clone, Debug, EnumString, Eq, PartialEq)]
pub enum DataType {
    String,
    Integer,
    Decimal,
    Float,
    Bool,
    Date,
    Time,
    DateTime,
    Table,
}
#[derive(Clone, Debug, Default)]
pub struct Table {
    pub table_name: String,
    pub row_delimiter: u8,
    // second row is data types
    pub data_types: Vec<DataType>,
    // third row is additional properties as strings
    pub additional_properties: Vec<String>,
    // fourth roe is column names, the last schema row
    pub column_names: Vec<String>,
    pub rows: Vec<Row>,
}

impl Table {
    /// table name is mandatory
    fn first_row_table_name(&mut self, rdr: &mut ReaderForQvs20) -> Result<(), Qvs20Error> {
        let next = rdr.next();
        if next.is_none() {
            return Err(Qvs20Error::ErrorInFirstRowTableName);
        } else {
            // unwrap cannot panic because of the conditionals before
            let result = unwrap!(next);
            if result.is_err() {
                return Err(Qvs20Error::ErrorInFirstRowTableName);
            } else {
                // unwrap cannot panic because of the conditionals before
                if let Token::Field(table_name) = unwrap!(result) {
                    let table_name = Self::unescape(table_name);
                    self.table_name = table_name;
                } else {
                    return Err(Qvs20Error::ErrorInFirstRowTableName);
                }
            }
        }
        Ok(())
    }
    /// row delimiter is mandatory
    fn first_row_row_delimiter(&mut self, rdr: &mut ReaderForQvs20) -> Result<(), Qvs20Error> {
        let next = rdr.next();
        if next.is_none() {
            return Err(Qvs20Error::ErrorInFirstRowRowDelimiter);
        } else {
            // unwrap cannot panic because of the conditionals before
            let result = unwrap!(next);
            if result.is_err() {
                return Err(Qvs20Error::ErrorInFirstRowRowDelimiter);
            } else {
                // unwrap cannot panic because of the conditionals before
                if let Token::RowDelimiter(row_delimiter) = unwrap!(result) {
                    self.row_delimiter = row_delimiter;
                } else {
                    return Err(Qvs20Error::ErrorInFirstRowRowDelimiter);
                }
            }
        }
        Ok(())
    }
    /// first data type is mandatory, next are optional
    fn second_row_1st_data_type(&mut self, rdr: &mut ReaderForQvs20) -> Result<(), Qvs20Error> {
        let next = rdr.next();
        if next.is_none() {
            return Err(Qvs20Error::ErrorInSecondRowDataType);
        } else {
            // unwrap cannot panic because of the conditionals before
            let result = unwrap!(next);
            if result.is_err() {
                return Err(Qvs20Error::ErrorInSecondRowDataType);
            } else {
                // unwrap cannot panic because of the conditionals before
                if let Token::Field(data_type) = unwrap!(result) {
                    let res_str_data_type = String::from_utf8(data_type.to_vec());
                    if res_str_data_type.is_err(){
                        return Err(Qvs20Error::ErrorInSecondRowDataType);
                    }else{
                        // unwrap cannot panic because of the conditionals before
                        let str_data_type=unwrap!(res_str_data_type);
                        let res_data_type = DataType::from_str(&str_data_type);
                        if res_data_type.is_err(){
                            return Err(Qvs20Error::ErrorInSecondRowDataType);
                        }else{
                            // unwrap cannot panic because of the conditionals before
                            let data_type=unwrap!(res_data_type);
                            self.data_types.push(data_type);
                        }
                    }
                } else {
                    return Err(Qvs20Error::ErrorInSecondRowDataType);
                }
            }
        }
        Ok(())
    }
    /// second and next data types are optional
    /// Option::None means end of the row
    fn second_row_next_data_type(&mut self, rdr: &mut ReaderForQvs20) -> Option<Result<(), Qvs20Error>> {
        let next = rdr.next();
        if next.is_none() {
            return Some(Err(Qvs20Error::ErrorInSecondRowDataType));
        } else {
            // unwrap cannot panic because of the conditionals before
            let result = unwrap!(next);
            if result.is_err() {
                return Some(Err(Qvs20Error::ErrorInSecondRowDataType));
            } else {
                // unwrap cannot panic because of the conditionals before
                if let Token::Field(data_type) = unwrap!(result) {
                    let res_str_data_type = String::from_utf8(data_type.to_vec());
                    if res_str_data_type.is_err(){
                        return Some(Err(Qvs20Error::ErrorInSecondRowDataType));
                    }else{
                        // unwrap cannot panic because of the conditionals before
                        let str_data_type=unwrap!(res_str_data_type);
                        let res_data_type = DataType::from_str(&str_data_type);
                        if res_data_type.is_err(){
                            return Some(Err(Qvs20Error::ErrorInSecondRowDataType));
                        }else{
                            // unwrap cannot panic because of the conditionals before
                            let data_type=unwrap!(res_data_type);
                            self.data_types.push(data_type);
                        }
                    }
                } else {
                    // Token::row_delimiter - end of the row
                    return None;
                }
            }
        }
        Some(Ok(()))
    }

    pub fn from_qvs20_with_schema(input: &[u8]) -> Result<Table, Qvs20Error> {
        let mut table = Table::default();
        let mut rdr = ReaderForQvs20::new(input);

        // first row: table name and row delimiter
        table.first_row_table_name(&mut rdr)?;
        table.first_row_row_delimiter(&mut rdr)?;
        table.second_row_1st_data_type(&mut rdr)?;
        loop{ 
            let opt = table.second_row_next_data_type(&mut rdr);
            if opt.is_none(){
                // row delimiter
                break;
            }
        }

        //return
        Ok(table)
    }

    /// unescape the qvs20 special 6 characters
    /// \\ Backslash character
    /// \[ Left Square Bracket
    /// \] Right Square Bracket
    /// \n New line
    /// \r Carriage return
    /// \t Tab
    pub fn unescape(field_value: &[u8]) -> String {
        //println!("unescape: {}", unwrap!(String::from_utf8(field_value.to_vec())));
        let mut ret = String::with_capacity(field_value.len());
        let mut start_pos = 0;
        while let Some(pos) = ReaderForQvs20::find_u8_from(field_value, start_pos, b'\\') {
            let end_pos = pos;
            // push the normal characters
            let str_value = unwrap!(String::from_utf8(field_value[start_pos..end_pos].to_vec()));
            ret.push_str(&str_value);
            // unescape the special character
            match field_value[end_pos + 1] {
                b'\\' => ret.push('\\'),
                b'[' => ret.push('['),
                b']' => ret.push(']'),
                b'n' => ret.push('\n'),
                b'r' => ret.push('\r'),
                b't' => ret.push('\t'),
                _ => ret.push('?'),
            }
            start_pos = end_pos + 2;
        }
        // push the last part of normal characters
        let end_pos = field_value.len();
        let str_value = unwrap!(String::from_utf8(field_value[start_pos..end_pos].to_vec()));
        ret.push_str(&str_value);

        // return
        ret
    }
}
#[cfg(test)]
mod test {
    use super::*;
    //use unwrap::unwrap;
    #[test]
    pub fn test_01() {
        // one byte characters
        let unescaped = Table::unescape(b"one");
        assert_eq!(unescaped, "one");
        // non one byte characters
        let unescaped = Table::unescape("čćšß€".as_bytes());
        assert_eq!(unescaped, "čćšß€");
        // qvs20 escape sequences
        let unescaped = Table::unescape(r"1\[2\]3\\4\r5\n6\t".as_bytes());
        assert_eq!(unescaped, "1[2]3\\4\r5\n6\t");
    }
    //#[test]
    pub fn test_02() {
        let s = r"[table-name]
[String][String]
[][]
[name][description]
[name_1][description_1]
[name_2][description_2]
[name_3][unescape\\\[\]\nNewLine]
";
        let table = unwrap!(Table::from_qvs20_with_schema(s.as_bytes()));
        let table2 = Table {
            table_name: "table-name".to_string(),
            row_delimiter: b'\n',
            data_types: vec![DataType::String, DataType::String],
            additional_properties: vec![String::new(), String::new()],
            column_names: vec!["name".to_string(), "description".to_string()],
            rows: vec![
                Row {
                    values: vec![
                        Value::String("name_1".to_string()),
                        Value::String("description_1".to_string()),
                    ],
                },
                Row {
                    values: vec![
                        Value::String("name_2".to_string()),
                        Value::String("description_2".to_string()),
                    ],
                },
                Row {
                    values: vec![
                        Value::String("name_3".to_string()),
                        Value::String(
                            r"unescape\[]
NewLine"
                                .to_string(),
                        ),
                    ],
                },
            ],
        };

        assert_eq!(format!("{:?}", table), format!("{:?}", table2));
    }
    #[test]
    pub fn test_03() {
        // error in first row - table_name
        let s = r"";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInFirstRowTableName);

        let s = r"bad-formed text";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInFirstRowTableName);

        let s = r"[bad-formed text";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInFirstRowTableName);

        let s = r"[no row delimiter]";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInFirstRowRowDelimiter);

        let s = "[row delimiter too big]\n\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInFirstRowRowDelimiter);

        // error in second row
        // good table_name, but no 2nd row
        let s = "[good name]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInSecondRowDataType);

        let s = "[good name]\n[String";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20Error::ErrorInSecondRowDataType);
        
    }
}
