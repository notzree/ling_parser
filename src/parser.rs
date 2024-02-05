use std::{collections::HashMap, hash::Hash};

use crate::constants::*;

#[derive(Debug)]
enum ParseError {
    RootNotObject,
    UnexpectedToken,
    OtherError(String),
    EmptyTokenList,
}

#[derive(Debug)]
enum JsonValue {
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
    Other(String),
}

pub fn parse(
    tokens: Vec<TokenType>,
    is_root: bool,
) -> Result<(TokenType, Vec<TokenType>), ParseError> {
    //todo: implement parser logic
    if tokens.len() == 0 {
        return Err(ParseError::EmptyTokenList);
    }

    let t = tokens.get(0).ok_or(ParseError::UnexpectedToken)?;
    if is_root {
        //Root must be object
        match t {
            TokenType::Char(c) => {
                if *c == JSON_LEFTBRACE {
                    return parse_object(tokens);
                }
            }
            _ => return Err(ParseError::RootNotObject),
        }
    }
    match t {
        TokenType::Char(c) => {
            if *c == JSON_LEFTBRACE {
                return parse_object(tokens);
            } else if *c == JSON_LEFTBRACKET {
                return parse_array(tokens);
            } else {
                return Ok((*t, tokens[1..].to_vec()));
            }
        }
        _ => return Ok((*t, tokens[1..].to_vec())),
    }
}

pub fn parse_object(tokens: Vec<TokenType>) -> Result<(TokenType, Vec<TokenType>), ParseError> {
    let mut json_object = HashMap::new();

    //Base case we encounter JSON_RIGHTBRACE meaning end of object:
    let t = tokens.get(0).ok_or(ParseError::UnexpectedToken)?;
    if let TokenType::Char(c) = t {
        if *c == JSON_RIGHTBRACE {
            return Ok((TokenType::Object(json_object), tokens[1..].to_vec()));
        }
    }
    

pub fn parse_array(tokens: Vec<TokenType>) -> Result<(TokenType, Vec<TokenType>), ParseError> {
    let mut json_array = HashMap::new();
}
