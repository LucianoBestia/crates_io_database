// qvs20_table_mod

// The table structure is very flexible, because it is defined in runtime.
// A lot of times the table is used just as an intermediary,
// and don't need a fixed Rust struct in compile time.
// It means that sometimes a change in the table does not dictate change in source code and compiling.

use crate::*;

use crate::qvs20_reader_mod::*;

//use strum;
use std::str::FromStr;
use strum_macros::EnumString;
use thiserror::Error;
use unwrap::unwrap;

#[macro_export]
macro_rules! unwrap_field_or_error(
    ($token:expr, $err:expr) => (
        match $token{
            crate::qvs20_reader_mod::Token::Field(p) => p,
            crate::qvs20_reader_mod::Token::RowDelimiter(_r) =>  return $err,
        };
    );
);
#[macro_export]
macro_rules! unwrap_row_delimiter_or_error(
    ($token:expr, $err:expr) => (
        match $token{
            crate::qvs20_reader_mod::Token::Field(_p) =>  return $err,
            crate::qvs20_reader_mod::Token::RowDelimiter(r) => r,
        };
    );
);

#[derive(Error, Debug)]
pub enum Qvs20ErrorTable {
    // table
    #[error("Error: {msg}")]
    Error { msg: String },
    #[error("Error: {msg} {source}")]
    ErrorWithSource {
        source: Qvs20ErrorReader,
        msg: String,
    },
    #[error("Error in third row: additional properties.")]
    ErrorInThirdRowAdditionalProperties,
    #[error("Error in fourth row: column names.")]
    ErrorInFourthRowColumnNames,
    #[error("Error in data row.")]
    ErrorInDataRow { row_number: usize },
    //#[error("unknown error")]
    //Unknown,
}

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
    /// first row: table name and row_delimiter are mandatory
    fn first_row_table_name(&mut self, rdr: &mut ReaderForQvs20) -> Result<(), Qvs20ErrorTable> {
        let result = match rdr.next() {
            Some(p) => p,
            None => {
                return Err(Qvs20ErrorTable::Error {
                    msg: "first row is empty.".to_string(),
                })
            }
        };
        let token = match result {
            Ok(p) => p,
            Err(e) => {
                return Err(Qvs20ErrorTable::ErrorWithSource {
                    source: e,
                    msg: "first row table name.".to_string(),
                })
            }
        };
        let table_name = match token {
            Token::Field(f) => f,
            Token::RowDelimiter(r) => {
                return Err(Qvs20ErrorTable::Error {
                    msg: format!(
                        "first row, expected Field found Row delimiter: {:?}.",
                        r
                    ),
                })
            }
        };
        self.table_name = Self::unescape(table_name);
        // row delimiter is mandatory
        let result = match rdr.next() {
            Some(p) => p,
            None => {
                return Err(Qvs20ErrorTable::Error {
                    msg: "first row missing row delimiter.".to_string(),
                })
            }
        };
        let token = match result {
            Ok(p) => p,
            Err(e) => {
                return Err(Qvs20ErrorTable::ErrorWithSource {
                    source: e,
                    msg: "first row ".to_string(),
                })
            }
        };
        let row_delimiter = match token {
            Token::Field(f) => {
                return Err(Qvs20ErrorTable::Error {
                    msg: "first row expected Row delimiter found Field.".to_string(),
                })
            }
            Token::RowDelimiter(r) => r,
        };
        self.row_delimiter = row_delimiter;

        Ok(())
    }

    /// second row: data types
    /// Option::None means end of the row
    fn second_row_data_types(
        &mut self,
        rdr: &mut ReaderForQvs20,
    ) -> Option<Result<(), Qvs20ErrorTable>> {
        let result = unwrap_option_or_error!(
            rdr.next(),
            Some(Err(Qvs20ErrorTable::Error{
                msg: "Missing mandatory second row - data types.".to_string(),
            }))
        );
        let token = match result {
            Ok(p) => p,
            Err(e) => {
                return Some(Err(Qvs20ErrorTable::ErrorWithSource {
                    source: e,
                    msg: "second row ".to_string(),
                }))
            }
        };
        // could be field or row_delimiter
        if let Token::RowDelimiter(r) = token {
            // row delimiter must be the same
            if r != self.row_delimiter {
                return Some(Err(Qvs20ErrorTable::Error{
                    msg: format!("second row wrong row delimiter:{:?} instead of {:?}",r, self.row_delimiter),
                }));
            }
            //end of row
            return None;
        }
        let data_type = match token {
            Token::Field(f) => f,
            Token::RowDelimiter(r) => {
                return Some(Err(Qvs20ErrorTable::Error {
                    msg: format!(
                        "second row, expected Field found Row delimiter: {:?}.",
                        r
                    ),
                }))
            }
        };
        let data_type = match String::from_utf8(data_type.to_vec()) {
            Ok(p) => p,
            Err(e) => {
                return Some(Err(Qvs20ErrorTable::Error {
                    msg: format!("second row {}",e),
                }))
            }
        };
        let data_type = match DataType::from_str(&data_type) {
            Ok(p) => p,
            Err(e) => {
                return Some(Err(Qvs20ErrorTable::Error {
                    msg: format!("second row {}", e),
                }))
            }
        };
        self.data_types.push(data_type);
        // return
        Some(Ok(()))
    }
    /// third row additional properties
    /// Option::None means end of the row
    fn third_row_additional_properties(
        &mut self,
        rdr: &mut ReaderForQvs20,
    ) -> Option<Result<(), Qvs20ErrorTable>> {
        let result = unwrap_option_or_error!(
            rdr.next(),
            Some(Err(Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties))
        );
        let token = unwrap_result_or_error!(
            result,
            Some(Err(Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties))
        );
        // could be field or row_delimiter
        if let Token::RowDelimiter(r) = token {
            // row delimiter must be the same
            if r != self.row_delimiter {
                return Some(Err(Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties));
            }
            //end of row
            return None;
        }
        let additional_property = unwrap_field_or_error!(
            token,
            Some(Err(Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties))
        );
        self.additional_properties
            .push(Self::unescape(additional_property));
        // return
        Some(Ok(()))
    }
    /// fourth row column names
    /// Option::None means end of the row
    fn fourth_row_column_names(
        &mut self,
        rdr: &mut ReaderForQvs20,
    ) -> Option<Result<(), Qvs20ErrorTable>> {
        let result = unwrap_option_or_error!(
            rdr.next(),
            Some(Err(Qvs20ErrorTable::ErrorInFourthRowColumnNames))
        );
        let token = unwrap_result_or_error!(
            result,
            Some(Err(Qvs20ErrorTable::ErrorInFourthRowColumnNames))
        );
        // could be field or row_delimiter
        if let Token::RowDelimiter(r) = token {
            // row delimiter must be the same
            if r != self.row_delimiter {
                return Some(Err(Qvs20ErrorTable::ErrorInFourthRowColumnNames));
            }
            //end of row
            return None;
        }
        let column_name = unwrap_field_or_error!(
            token,
            Some(Err(Qvs20ErrorTable::ErrorInFourthRowColumnNames))
        );
        let column_name = Self::unescape(column_name);
        // names of columns must be unique
        for name in self.column_names.iter() {
            if name == &column_name {
                return Some(Err(Qvs20ErrorTable::ErrorInFourthRowColumnNames));
            }
        }
        self.column_names.push(column_name);
        // return
        Some(Ok(()))
    }
    /// create an object in memory from a qvs20 string in [u8] format
    pub fn from_qvs20_with_schema(input: &[u8]) -> Result<Table, Qvs20ErrorTable> {
        let mut table = Table::default();
        let mut rdr = ReaderForQvs20::new(input);

        // first row: table name and row delimiter
        table.first_row_table_name(&mut rdr)?;
        // second row: data types
        while let Some(result) = table.second_row_data_types(&mut rdr) {
            // if Err then propagate
            result?;
        }
        // must have at least one column
        if table.data_types.is_empty() {
            return Err(Qvs20ErrorTable::Error{
                msg:"second row, data types is empty.".to_string(),
            });
        }
        // third row - additional properties
        while let Some(result) = table.third_row_additional_properties(&mut rdr) {
            // if Err then propagate
            result?;
        }
        // third rows must have same number of columns as data_types
        if table.additional_properties.len() != table.data_types.len() {
            return Err(Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties);
        }
        // fourth row - column names
        while let Some(result) = table.fourth_row_column_names(&mut rdr) {
            // if Err then propagate
            result?;
        }
        // every row must have same number of columns as data_types
        if table.column_names.len() != table.data_types.len() {
            return Err(Qvs20ErrorTable::ErrorInFourthRowColumnNames);
        }
        // rows of data

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
    pub fn test_01_unescape() {
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
    pub fn test_03_all_wrong_in_schema() {
        
        // error in first row - table_name
        let s = r"";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: first row is empty.");

        let s = r"bad-formed text";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: first row table name. Error: The field must start with [. pos: 0");

        let s = r"[bad-formed text";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: first row table name. Error: Last bracket is missing. pos: 1");

        let s = r"[no row delimiter]";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: first row  Error: Last row delimiter is missing. pos: 18");

        let s = "[row delimiter too big]\n\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: first row  Error: The row delimiter has more than 1 byte. pos: 23");

        // error in second row - data types
        // good table_name, but no 2nd row
        let s = "[table name]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: Missing mandatory second row - data types.");

        let s = "[table name]\n[String";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: second row  Error: Last bracket is missing. pos: 14");

        let s = "[table name]\n[String]";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: second row  Error: Last row delimiter is missing. pos: 21");

        let s = "[table name]\n[String][Integer][Decimal]";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: second row  Error: Last row delimiter is missing. pos: 39");

        // wrong row delimiter
        let s = "[table name]\n[String][Integer][Decimal]1";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: second row wrong row delimiter:49 instead of 10");

        let s = "[table name]\n[String][Integer][Decimal]\n\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(), "Error: second row  Error: The row delimiter has more than 1 byte. pos: 39");

        // error in third row - additional properties
        let s = "[table name]\n[String][Integer][Decimal]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg.to_string(),"");
/*
        let s = "[table name]\n[String][Integer][Decimal]\n[prop";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(
            err_msg,
            Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties
        );

        let s = "[table name]\n[String][Integer][Decimal]\n[prop]";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(
            err_msg,
            Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties
        );

        let s = "[table name]\n[String][Integer][Decimal]\n[prop][prop]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(
            err_msg,
            Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties
        );
        // wrong row delimiter
        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]1";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(
            err_msg,
            Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties
        );
        // row delimiter must be only 1 byte
        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(
            err_msg,
            Qvs20ErrorTable::ErrorInThirdRowAdditionalProperties
        );

        // fourth row: column names
        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);

        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n[name";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);

        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n[name]";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);

        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n[name1][name2]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);
        // column names must be unique
        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n[name1][name1][name1]\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);
        // wrong row delimiter
        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n[name1][name2][name3]1";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);

        let s = "[table name]\n[String][Integer][Decimal]\n[prop1][prop2][prop3]\n[name1][name2][name3]\n\n";
        let err_msg = Table::from_qvs20_with_schema(s.as_bytes()).unwrap_err();
        assert_eq!(err_msg, Qvs20ErrorTable::ErrorInFourthRowColumnNames);

        // rows of data
        */
    }
}
