// utils_mod

use unwrap::unwrap;

/// parse semver ex. 12.99.88alpha
pub fn parse_semver(text: &str) -> (usize, usize, usize) {
    let pos = 0;
    let (major, pos) = parse_next_number(&text, pos);
    // jump over dot
    let pos = pos + 1;
    let (minor, pos) = parse_next_number(&text, pos);
    // jump over dot
    let pos = pos + 1;
    let (patch, _pos) = parse_next_number(&text, pos);
    // return
    (major, minor, patch)
}

/// parse next characters until is numeric or end
fn parse_next_number(text: &str, pos: usize) -> (usize, usize) {
    let mut pos = pos;
    let mut number = String::new();
    let mut one_char = text[pos..pos + 1].chars().next().unwrap();
    while one_char.is_numeric() {
        number.push(one_char);
        pos += 1;
        if pos > text.len() - 1 {
            break;
        }
        one_char = text[pos..pos + 1].chars().next().unwrap();
    }
    let number: usize = unwrap!(number.parse());
    // return
    (number, pos)
}

/// version for sorting
pub fn version_for_sorting(version: &str) -> String {
    let (major, minor, patch) = parse_semver(version);
    let version_for_sorting = format!("{:09}.{:09}.{:09}", major, minor, patch,);
    // return
    version_for_sorting
}

