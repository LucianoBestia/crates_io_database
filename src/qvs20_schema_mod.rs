//! schema_mod

use strum_macros::{Display, EnumString};
// AsRefStr, EnumString
//use unwrap::unwrap;
//use anyhow::anyhow;
//use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Schema {
    // first row is table_name and row_delimiter
    pub table_name: String,
    // first row is table_name and row_delimiter
    pub row_delimiter: String,

    // private field
    cursor_pos: usize,
}

#[derive(Clone, Debug, Display, EnumString, Eq, PartialEq)]
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

/*
impl Schema {
    /// crate schema from string
    /// Schema contains 4 rows.
    pub fn new_from_str(schema_str: &str) -> anyhow::Result<Schema> {
        // [table_name]row_delimiter
        // [String][Integer]
        // [add_prop][add_prop]
        // [Country][Population]

        if schema_str.is_empty(){
            return Err(anyhow!("The schema must not be empty."));
        }

        // Parsing will get vectors of strings for one row.
        // Parsing is simple because there is only Strings in this data.
        // This code looks similar to parsing qvs20 data, but it is not the same.
        // It is much simplified and specialized.

        cursor_pos=0;

        get_next_field(schema_str, cursor_pos)


// row delimiter can be LF, 1, 2, 3, ...
        // we find it quickly after the table_name
        // We could just look at the space between fields.
        // That is between end and start ][ delimiters.
        // Only the row delimiter is allowed there.
        // It means the row finishes after the delimiter.
        // it can be the end of the file, or a new row starting with [
        let (data_types, cursor_pos) = Self::get_vec_of_strings(schema_str, 0);
        // the returned cursor_pos is the new position
        let (additional_properties, cursor_pos) = Self::get_vec_of_strings(schema_str, cursor_pos);
        let (column_names, _cursor_pos) = Self::get_vec_of_strings(schema_str, cursor_pos);

        // this vec of string need to be transformed to vec od DataType enum
        let data_types = data_types
            .iter()
            .map(|x| unwrap!(DataType::from_str(x)))
            .collect();

        //return
        Schema {
            data_types,
            additional_properties,
            column_names,
        }
    }
    /// schema to string
    pub fn to_string(&self, row_delimiter: &str) -> String {
        let mut schema_str = String::with_capacity(1000);
        for data_type in &self.data_types {
            schema_str.push('[');
            schema_str.push_str(&data_type.to_string());
            schema_str.push(']');
        }
        schema_str.push_str(row_delimiter);
        for property in &self.additional_properties {
            schema_str.push('[');
            schema_str.push_str(&property.to_string());
            schema_str.push(']');
        }
        schema_str.push_str(row_delimiter);
        for column_name in &self.column_names {
            schema_str.push('[');
            schema_str.push_str(&column_name);
            schema_str.push(']');
        }
        schema_str.push_str(row_delimiter);
        //return
        schema_str
    }

    pub fn get_next_field(schema_str: &str, cursor_pos: usize)-> anyhow::Result<(Cow<String>, usize)> {
        if unwrap!(schema_str.bytes().nth(cursor_pos)) != b'['{
            return Err(anyhow!("The field at pos {} must start with [.", cursor_pos));
        }

    }

    // get vec of string for data_types, additional_properties and column_names
    // returns the cursor_pos of the new position
    pub fn get_vec_of_strings(schema_str: &str, cursor_pos: usize) -> anyhow::Result<(Vec<String>, usize)> {
        // string slices are positioned by bytes(), not by char()
        // Working with Vec<Byte> it faster than working with String.
        // because of the utf-8 crazy rules. We just ignore them.
        // it is safe because at the end we transform it in Strings.
        // that is the moment when Rust checks that the utf8 is correct.
        // And it should be. We disturb only the delimiters and leave the rest alone.
        // If is a string then we want to unescape the special characters.

        if unwrap!(schema_str.bytes().nth(cursor_pos)) != b'['{
            return Err(anyhow!("The schema row at pos {} must start with [.", cursor_pos));
        }
        let mut vec_of_strings: Vec<String> = vec![];
        let mut found_row_end_delimiter = false;
        let mut between_end_and_start = false;
        // shadowing !
        let mut cursor_pos: usize = cursor_pos;
        let mut is_escaped = false;
        let mut utf8_text = vec![];

        for xu8 in schema_str[cursor_pos..].as_bytes() {
            if is_escaped == true {
                //escaped sequence is always exact 2 bytes
                utf8_text.push(*xu8);
                is_escaped = false;
            } else {
                if xu8 == &b'\\' {
                    //always start of the escaped sequence. There is always exactly 2 bytes.
                    is_escaped = true;
                } else if xu8 == &b'[' {
                    if found_row_end_delimiter == true {
                        // end of the row delimiter and end of the row
                        // in case of the last row, the end of file comes first
                        // return the cursor position -> for the next row
                        break;
                    } else {
                        // start of new utf8_text
                        between_end_and_start = false;
                    }
                } else if xu8 == &b']' {
                    // end of utf8_text
                    let text = unwrap!(std::str::from_utf8(&utf8_text));
                    vec_of_strings.push(text.to_string());
                    utf8_text = vec![];
                    between_end_and_start = true;
                } else if between_end_and_start == true {
                    // in this space between ] and [ is allowed only the row_delimiter
                    // it can be various bytes long LF, 1, 2, 3,...
                    // here we don't need to do anything with it. Simplified.
                    found_row_end_delimiter = true;
                } else {
                    // fill the result vector with bytes, one by one
                    // the sequence to be escaped are already solved and don't come to this point
                    utf8_text.push(*xu8);
                }
            }
            cursor_pos += 1;
        }
        if found_row_end_delimiter == false{
            return Err(anyhow!("The row does not end with row_delimiter pos {}.", cursor_pos));
        }
        //return
        Ok((vec_of_strings, cursor_pos))
    }
}
*/
