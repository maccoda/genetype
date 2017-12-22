use std::ascii::AsciiExt;
use std::ops::Index;

pub fn snake_case_to_capitalized(s: &str) -> String {
    let mut result = String::from(s.index(0..1)).to_ascii_uppercase();
    let mut to_capitalize = false;
    for c in s.chars().skip(1) {
        if c.is_alphanumeric() {
            let next_char = if to_capitalize { c.to_ascii_uppercase() } else { c };
            result.push(next_char);
            to_capitalize = false;
        } else if c.eq(&'_') {
            to_capitalize = true;
        }
    }
    result
}

pub fn camel_case_to_snake_case(s: String) -> String {
    unimplemented!()
}

pub fn plural_to_singular(s: &str) -> String {
    if s.ends_with("s") {
        s.index(..s.len() - 1).to_owned()
    } else {
        s.to_owned()
    }
}


pub fn capitalize_string(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect()
    }
}