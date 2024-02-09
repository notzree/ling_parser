use std::collections::HashMap;

use crate::constants::*;

#[derive(Debug)]
pub enum ParseError {
    RootNotObject,
    UnexpectedToken,
    OtherError(String),
    EmptyTokenList,
}

#[derive(Debug)]
pub enum JsonValue {
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
    Other(String),
}

pub enum ParseResult {
    Array(Vec<ParseResult>),
    Object(HashMap<String, ParseResult>),
    String(String),
    Char(char),
    Number(Number), // Assuming JSON numbers are represented as
    Boolean(bool),
    Null,
}

pub fn parse(
    tokens: Vec<TokenType>,
    is_root: bool,
) -> Result<(ParseResult, Vec<TokenType>), ParseError> {
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
                return Ok((ParseResult::Char(*c), tokens[1..].to_vec()));
            }
        }
        TokenType::Str(s) => Ok((ParseResult::String(s.to_string()), tokens[1..].to_vec())),
        TokenType::Number(n) => Ok((ParseResult::Number(n.clone()), tokens[1..].to_vec())),
        TokenType::Boolean(b) => Ok((ParseResult::Boolean(*b), tokens[1..].to_vec())),
        TokenType::Null() => Ok((ParseResult::Null, tokens[1..].to_vec())),
        // _ => return Ok((ParseResult::Char(*t), tokens[1..].to_vec())),
    }
}

pub fn parse_object(
    mut tokens: Vec<TokenType>,
) -> Result<(ParseResult, Vec<TokenType>), ParseError> {
    let mut json_object = HashMap::new();
    let start_token = tokens.remove(0);
    match start_token {
        TokenType::Char(c) if c == JSON_RIGHTBRACE => {
            return Ok((ParseResult::Object(json_object), tokens))
        }
        start_token => {
            while !tokens.is_empty() {
                let json_key = &start_token;
                match json_key {
                    TokenType::Str(json_key) => {
                        //We encountered a string, meaning this should be a key which we have already removed (line 70)
                        let next_token = tokens.remove(0);
                        if next_token != TokenType::Char(JSON_COLON) {
                            return Err(ParseError::UnexpectedToken);
                        }
                        let (json, remaining_tokens) = parse(tokens, false)?;
                        json_object.insert(json_key.to_string(), json);
                        tokens = remaining_tokens;
                        let next_token = tokens.get(0).ok_or(ParseError::UnexpectedToken)?;

                        match next_token {
                            TokenType::Char(c) if *c == JSON_RIGHTBRACE => {
                                //Reached end of object
                                return Ok((
                                    ParseResult::Object(json_object),
                                    tokens[1..].to_vec(),
                                ));
                            }
                            TokenType::Char(c) if *c == JSON_COMMA => {
                                tokens.remove(0);
                            }
                            _ => {
                                return Err(ParseError::UnexpectedToken);
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken);
                    }
                }
            }
        }
    }
    return Ok((ParseResult::Object(json_object), tokens));
}

pub fn parse_array(
    mut tokens: Vec<TokenType>,
) -> Result<(ParseResult, Vec<TokenType>), ParseError> {
    let mut json_array: Vec<ParseResult> = Vec::new();
    let start_token = tokens.remove(0);
    match start_token {
        TokenType::Char(c) if c == JSON_RIGHTBRACKET => {
            return Ok((ParseResult::Array(json_array), tokens));
        }
        TokenType::Char(c) => {
            while !tokens.is_empty() {
                let (json, remaining_tokens) = parse(tokens, false)?;
                json_array.push(json);
                tokens = remaining_tokens;
                let next_token = tokens.get(0).ok_or(ParseError::UnexpectedToken)?;
                match next_token {
                    TokenType::Char(c) if *c == JSON_RIGHTBRACKET => {
                        //Reached end of array
                        return Ok((ParseResult::Array(json_array), tokens[1..].to_vec()));
                    }
                    TokenType::Char(c) if *c == JSON_COMMA => {
                        tokens.remove(0);
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken);
                    }
                }
            }
            return Ok((ParseResult::Array(json_array), tokens));
        }
        _ => return Err(ParseError::UnexpectedToken),
    }
}
