// region: lmake_readme include "README.md" //! A

// endregion: lmake_readme include "README.md" //! A

// region: Clippy
#![deny(unused_must_use)]

// CONS: Unnecessary code.
// PROS: more readable without knowing that the type is bool.
#[allow(clippy::bool_comparison)]
// endregion: Clippy

// region: mod, extern and use statements
mod extract_and_save_mod;
mod qvs20_reader_mod;
mod qvs20_schema_mod;
mod qvs20_table_mod;
mod qvs20_writer_mod;
mod utils_mod;

//use unwrap::unwrap;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
//use ansi_term::Style;
use clap::App;
use std::env;
// endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let _arguments = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    // extract_and_save_mod::extract_and_save();
    //use qvs20_table_mod::*;
    //use std::str;
    let s = r"[crates \] table]
[String][String][String][Integer][String]
[][][][][]
[name][description][repository][id][last_version]
[name_1][A small git 1][https://github.com/ 1][1601][0.1.1]
[name_2][A small git 2][https://github.com/ 2][1602][0.1.2]
[name_3][A small git 3][https://github.com/ 3][1603][0.1.3]
[name_4][A small git 4][https://github.com/ 4][1604][0.1.4]
[name_5][ 5][https://github.com/ 5][1][0\[1\]\n5]
";
    let table = qvs20_table_mod::Table::from_qvs20_with_schema(s.as_bytes());
    dbg!(&table);
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion
