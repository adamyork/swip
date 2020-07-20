use crate::parser::token_parser::TokenParser;
use crate::token::token_type::TokenTypes;
use crate::token::tokenizer::Token;
use crate::token::tokenizer::Tokenizer;

#[test]
fn test_token_regex() {
    let is_whitespace = TokenTypes::WHITESPACE.tokens().regex().is_match(" ");
    let is_number = TokenTypes::NUMBER.tokens().regex().is_match("1");
    let is_string = TokenTypes::IDENTIFIER
        .tokens()
        .regex()
        .is_match("someString");
    let is_lparen = TokenTypes::LPAREN.tokens().regex().is_match("(");
    let is_paren = TokenTypes::RPAREN.tokens().regex().is_match(")");
    let is_print = TokenTypes::PRINT.tokens().regex().is_match("#");
    assert_eq!(is_whitespace, true);
    assert_eq!(is_number, true);
    assert_eq!(is_string, true);
    assert_eq!(is_lparen, true);
    assert_eq!(is_paren, true);
    assert_eq!(is_print, true);
}

#[test]
fn test_tokenizer_number() {
    let tokenizer = Tokenizer { debug: false };
    let tokens: Vec<Token> = tokenizer.tokenize("**$ number1 27\n**$ number2 42");
    assert_eq!(tokens.len(), 6);
    let number_token_1 = tokens.get(0).unwrap();
    let identifier_token_1 = tokens.get(1).unwrap();
    let value_token_1 = tokens.get(2).unwrap();
    let number_token_2 = tokens.get(3).unwrap();
    let identifier_token_2 = tokens.get(4).unwrap();
    let value_token_2 = tokens.get(5).unwrap();
    assert_eq!(number_token_1.value, "**$");
    assert_eq!(identifier_token_1.value, "number1");
    assert_eq!(value_token_1.value, "27");
    assert_eq!(number_token_2.value, "**$");
    assert_eq!(identifier_token_2.value, "number2");
    assert_eq!(value_token_2.value, "42");
}

#[test]
fn test_tokenizer_string() {
    let tokenizer = Tokenizer { debug: false };
    let tokens: Vec<Token> = tokenizer.tokenize("**\" string1 \"hi\"\n**\" string2 \"world\"");
    assert_eq!(tokens.len(), 6);
    let string_token_1 = tokens.get(0).unwrap();
    let identifier_token_1 = tokens.get(1).unwrap();
    let value_token_1 = tokens.get(2).unwrap();
    let string_token_2 = tokens.get(3).unwrap();
    let identifier_token_2 = tokens.get(4).unwrap();
    let value_token_2 = tokens.get(5).unwrap();
    assert_eq!(string_token_1.value, "**\"");
    assert_eq!(identifier_token_1.value, "string1");
    assert_eq!(value_token_1.value, "\"hi\"");
    assert_eq!(string_token_2.value, "**\"");
    assert_eq!(identifier_token_2.value, "string2");
    assert_eq!(value_token_2.value, "\"world\"");
}

#[test]
fn test_tokenizer_boolean() {
    let tokenizer = Tokenizer { debug: false };
    let tokens: Vec<Token> = tokenizer.tokenize("**b boolean1 true\n**b boolean2 false");
    assert_eq!(tokens.len(), 6);
    let boolean_token_1 = tokens.get(0).unwrap();
    let identifier_token_1 = tokens.get(1).unwrap();
    let value_token_1 = tokens.get(2).unwrap();
    let boolean_token_2 = tokens.get(3).unwrap();
    let identifier_token_2 = tokens.get(4).unwrap();
    let value_token_2 = tokens.get(5).unwrap();
    assert_eq!(boolean_token_1.value, "**b");
    assert_eq!(identifier_token_1.value, "boolean1");
    assert_eq!(value_token_1.value, "true");
    assert_eq!(boolean_token_2.value, "**b");
    assert_eq!(identifier_token_2.value, "boolean2");
    assert_eq!(value_token_2.value, "false");
}

#[test]
fn test_tokenizer_print() {
    let tokenizer = Tokenizer { debug: false };
    let tokens: Vec<Token> = tokenizer.tokenize("# \"hello\"\n**b boolean1 false\n# boolean1");
    assert_eq!(tokens.len(), 7);
    let print_token_1 = tokens.get(0).unwrap();
    let value_token_1 = tokens.get(1).unwrap();
    let boolean_token = tokens.get(2).unwrap();
    let identifier_token_1 = tokens.get(3).unwrap();
    let value_token = tokens.get(4).unwrap();
    let print_token_2 = tokens.get(5).unwrap();
    let identifier_token_2 = tokens.get(6).unwrap();
    assert_eq!(print_token_1.value, "#");
    assert_eq!(value_token_1.value, "\"hello\"");
    assert_eq!(boolean_token.value, "**b");
    assert_eq!(identifier_token_1.value, "boolean1");
    assert_eq!(print_token_2.value, "#");
    assert_eq!(identifier_token_2.value, "boolean1");
}
