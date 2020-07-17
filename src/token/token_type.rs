use std::fmt;

use regex::Regex;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
pub enum TokenTypes {
    WHITESPACE,
    NUMBER,
    IDENTIFIER,
    LPAREN,
    RPAREN,
    PRINT,
}

impl TokenTypes {
    pub const TYPE_MUTABLE_STRING: &'static str = "**\"";
    pub const TYPE_IMMUTABLE_STRING: &'static str = "*\"";
    pub const TYPE_MUTABLE_NUMBER: &'static str = "**$";
    pub const TYPE_IMMUTABLE_NUMBER: &'static str = "*$";
    //noinspection ALL
    pub fn tokens(self: Self) -> &'static TokenType {
        lazy_static! {
            static ref WHITESPACE: TokenType = TokenType {
                name: "whitespace",
                regex: Regex::new(r"\s").unwrap(),
            };
            static ref NUMBER: TokenType = TokenType {
                name: "number",
                regex: Regex::new(r"[0-9]").unwrap(),
            };
            static ref IDENTIFIER: TokenType = TokenType {
                name: "identifier",
                regex: Regex::new("\\*|\\$|[A-Za-z\"]").unwrap(),
            };
            static ref LPAREN: TokenType = TokenType {
                name: "lparen",
                regex: Regex::new(r"\(").unwrap(),
            };
            static ref RPAREN: TokenType = TokenType {
                name: "rparen",
                regex: Regex::new(r"\)").unwrap(),
            };
            static ref PRINT: TokenType = TokenType {
                name: "print",
                regex: Regex::new(r"\#").unwrap(),
            };
        }
        match self {
            TokenTypes::WHITESPACE => &WHITESPACE,
            TokenTypes::NUMBER => &NUMBER,
            TokenTypes::IDENTIFIER => &IDENTIFIER,
            TokenTypes::LPAREN => &LPAREN,
            TokenTypes::RPAREN => &RPAREN,
            TokenTypes::PRINT => &PRINT,
        }
    }
}

pub struct TokenType {
    pub name: &'static str,
    pub regex: Regex,
}

impl TokenType {
    pub fn regex(&self) -> &Regex {
        &self.regex
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}", &self.name)
    }
}
