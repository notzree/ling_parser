pub const JSON_QUOTE: char = '"';
pub static JSON_WHITESPACE: [char; 4] = [' ', '\n', '\t', '\r'];
pub const JSON_COMMA: char = ',';
pub const JSON_COLON: char = ':';
pub const JSON_LEFTBRACKET: char = '[';
pub const JSON_RIGHTBRACKET: char = ']';
pub const JSON_LEFTBRACE: char = '{';
pub const JSON_RIGHTBRACE: char = '}';
pub const JSON_SYNTAX: [char; 7] = [
    JSON_QUOTE,
    JSON_COMMA,
    JSON_COLON,
    JSON_LEFTBRACKET,
    JSON_RIGHTBRACKET,
    JSON_LEFTBRACE,
    JSON_RIGHTBRACE,
];
// This code snippet is illustrative and uses a static slice for simplicity
pub static NUMBER_CHARACTERS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '.', 'e', '+',
];

pub const TRUE_LENGTH: usize = "true".len();
pub const FALSE_LENGTH: usize = "false".len();
pub const NULL_LENGTH: usize = "null".len();

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i32),   // You can choose the integer type that best fits your needs (i32, i64, etc.)
    Float(f64), // Similarly, choose the floating-point precision you need (f32, f64)
}
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Str(String),
    Number(Number),
    Boolean(bool),
    Null, // Just represent Null as a variant without any data
    Char(char),
}
