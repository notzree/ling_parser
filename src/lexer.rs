// Breaks down input strings into tokens

use crate::constants::*;

fn lex_string(str: String) -> Result<(Option<String>, String), Box<dyn std::error::Error>> {
    let mut json_string = String::new();
    let mut itr = 0;
    if let Some(c) = str.chars().nth(0) {
        if c == JSON_QUOTE {
            itr += 1;
        } else {
            return Ok((None, str));
        }
    }
    for c in str.chars().skip(itr) {
        if c == JSON_QUOTE {
            return Ok((Some(json_string), str.chars().skip(itr + 1).collect()));
            // +1 to skip the quote
        }
        json_string.push(c);
        itr += 1;
    }
    Err("Expected end of string quote".into())
}

fn lex_number(str: String) -> Result<(Option<Number>, String), Box<dyn std::error::Error>> {
    let mut json_string = String::new();
    let mut remainder = str.chars().peekable();
    while let Some(&c) = remainder.peek() {
        if NUMBER_CHARACTERS.contains(&c) {
            json_string.push(c);
            remainder.next(); // Consume the character
        } else {
            break;
        }
    }

    // Convert the remainder back into a String
    let remainder_string: String = remainder.collect();

    // Try to parse the json_string as i32 first, then as f64 if i32 parsing fails
    let json_number = json_string
        .parse::<i32>()
        .map(|i| Some(Number::Int(i))) // If parsing as i32 succeeds, Map it as Numer::Int
        .or_else(|_| json_string.parse::<f64>().map(|f| Some(Number::Float(f)))) // If i32 parsing fails, try f64
        .unwrap_or(None); // If both fail, return None
    Ok((json_number, remainder_string))
}

fn lex_bool(str: String) -> Result<(Option<bool>, String), Box<dyn std::error::Error>> {
    let length = str.len();
    if length >= TRUE_LENGTH && &str[0..TRUE_LENGTH] == "true" {
        return Ok((Some(true), str.chars().skip(TRUE_LENGTH).collect()));
    } else if length >= FALSE_LENGTH && &str[0..FALSE_LENGTH] == "false" {
        return Ok((Some(false), str.chars().skip(FALSE_LENGTH).collect()));
    }
    Ok((None, str))
}

fn lex_null(str: String) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let length = str.len();
    if length >= NULL_LENGTH && &str[0..NULL_LENGTH] == "null" {
        return Ok((true, str.chars().skip(NULL_LENGTH).collect()));
    }
    return Ok((false, str));
}

pub fn lex(str: String) -> Result<Vec<TokenType>, Box<dyn std::error::Error>> {
    let mut input_string = str.clone();
    let mut tokens: Vec<TokenType> = Vec::new();
    while !input_string.is_empty() {
        //Try parsing as number
        let (json_number, return_string) = lex_number(input_string)?;
        input_string = return_string;
        if let Some(s) = json_number {
            tokens.push(TokenType::Number(s));
            continue;
        }

        //try to parse current input as string
        let (json_string, return_string) = lex_string(input_string)?;
        input_string = return_string;
        if let Some(s) = json_string {
            tokens.push(TokenType::Str(s));
            continue;
        }

        //Try parsing as boolean
        let (json_bool, return_string) = lex_bool(input_string)?;
        input_string = return_string;
        if let Some(s) = json_bool {
            tokens.push(TokenType::Boolean(s));
            continue;
        }
        //Try parsing as null
        let (is_null, return_string) = lex_null(input_string)?;
        input_string = return_string;
        if is_null {
            tokens.push(TokenType::Null);
            continue;
        }

        //Now check for JSON_SYNTAX...
        let c = input_string.chars().next().unwrap();
        if JSON_WHITESPACE.contains(&c) {
            input_string = input_string.chars().skip(1).collect();
            continue;
        } else if JSON_SYNTAX.contains(&c) {
            tokens.push(TokenType::Char(c));
            input_string = input_string.chars().skip(1).collect();
            continue;
        } else {
            return Err(format!("Invalid Character {}", c).into());
        }
    }
    Ok(tokens)
}
