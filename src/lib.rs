mod constants;
mod lexer;
mod parser;

mod tests {
    use crate::lexer::lex;

    use self::constants::*;

    use super::*;

    #[test]
    fn correctly_lexes_string() {
        let test_string = "{\"key\":\"value\"}".to_string();
        let result = lex(test_string);
        let expected = vec![
            TokenType::Char('{'),
            TokenType::Str("key".to_string()),
            TokenType::Char(':'),
            TokenType::Str("value".to_string()),
            TokenType::Char('}'),
        ];
        match result {
            Ok(r) => {
                assert_eq!(r, expected)
            }
            Err(_) => assert!(false),
        }
    }
    #[test]
    fn correctly_lexes_numbers() {
        let test_string = "{\"key1\":900,\"key2\":99.99}".to_string();
        let result = lex(test_string);
        let expected = vec![
            TokenType::Char('{'),
            TokenType::Str("key1".to_string()),
            TokenType::Char(':'),
            TokenType::Number(Number::Int(900)),
            TokenType::Char(','),
            TokenType::Str("key2".to_string()),
            TokenType::Char(':'),
            TokenType::Number(Number::Float(99.99)),
            TokenType::Char('}'),
        ];
        match result {
            Ok(r) => {
                assert_eq!(r, expected)
            }
            Err(_) => assert!(false),
        }
    }
    #[test]
    fn correctly_lexes_bool() {
        let test_string = "{\"do eces shower?\":false,\"Do eces stink?\":true}".to_string();
        let result = lex(test_string);
        let expected = vec![
            TokenType::Char('{'),
            TokenType::Str("do eces shower?".to_string()),
            TokenType::Char(':'),
            TokenType::Boolean(false),
            TokenType::Char(','),
            TokenType::Str("Do eces stink?".to_string()),
            TokenType::Char(':'),
            TokenType::Boolean(true),
            TokenType::Char('}'),
        ];
        match result {
            Ok(r) => {
                assert_eq!(r, expected)
            }
            Err(_) => assert!(false),
        }
    }
    #[test]
    fn correctly_lexes_nulls() {
        let test_string = "{\"null_value_one\":null,\"null_value_two\":\"Null\"}".to_string();
        let result = lex(test_string);
        let expected = vec![
            TokenType::Char('{'),
            TokenType::Str("null_value_one".to_string()),
            TokenType::Char(':'),
            TokenType::Null(),
            TokenType::Char(','),
            TokenType::Str("null_value_two".to_string()),
            TokenType::Char(':'),
            TokenType::Str("Null".to_string()),
            TokenType::Char('}'),
        ];
        match result {
            Ok(r) => {
                println!("{:?}", r);
                assert_eq!(r, expected)
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false)
            }
        }
    }
    #[test]
    fn correctly_lexes_nested_json() {
        let test_string = r#"{
            "nested_object": {"inner_key": "inner_value", "inner_number": 42.69},
            "nested_array": [true, null, {"array_object_key": "array_object_value"}]
        }"#
        .to_string();
        let result = lex(test_string);
        let expected = vec![
            TokenType::Char('{'),
            TokenType::Str("nested_object".to_string()),
            TokenType::Char(':'),
            TokenType::Char('{'),
            TokenType::Str("inner_key".to_string()),
            TokenType::Char(':'),
            TokenType::Str("inner_value".to_string()),
            TokenType::Char(','),
            TokenType::Str("inner_number".to_string()),
            TokenType::Char(':'),
            TokenType::Number(Number::Float(42.69)),
            TokenType::Char('}'),
            TokenType::Char(','),
            TokenType::Str("nested_array".to_string()),
            TokenType::Char(':'),
            TokenType::Char('['),
            TokenType::Boolean(true),
            TokenType::Char(','),
            TokenType::Null(),
            TokenType::Char(','),
            TokenType::Char('{'),
            TokenType::Str("array_object_key".to_string()),
            TokenType::Char(':'),
            TokenType::Str("array_object_value".to_string()),
            TokenType::Char('}'),
            TokenType::Char(']'),
            TokenType::Char('}'),
        ];
        match result {
            Ok(r) => assert_eq!(r, expected),
            Err(_) => assert!(false, "Lexer failed to lex nested JSON correctly."),
        }
    }
}
