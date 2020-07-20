use strum::IntoEnumIterator;

use crate::token::data_types::DataTypes;
use crate::token::token_type::TokenTypes;

pub struct Tokenizer {
    pub debug: bool
}

impl Tokenizer {
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let split = input.split("");
        let mut symbols: Vec<&str> = split.collect();
        let mut tokens: Vec<Token> = Vec::new();
        while !symbols.is_empty() {
            let symbol = symbols.remove(0);
            'inner: for t_type in TokenTypes::iter() {
                //println!("{:?}", token_type);
                let is_match = t_type.tokens().regex().is_match(symbol);
                if is_match {
                    if t_type == TokenTypes::WHITESPACE {
                        continue 'inner;
                    }
                    //println!("found a {:?} token with value {}", token_type, symbol);
                    let after = Tokenizer::eat(&mut symbols);
                    let mut next = symbol.to_string();
                    next.push_str(after.as_str());
                    let mut next_data_type = DataTypes::NonDataType;
                    for d_type in DataTypes::iter() {
                        let d_type_str = d_type.as_str();
                        if next.eq(d_type_str) {
                            next_data_type = d_type;
                        }
                    }
                    let next_token: Token = Token {
                        token_type: t_type,
                        data_type: next_data_type,
                        value: next.clone(),
                    };
                    tokens.push(next_token);
                    if self.debug {
                        println!("token created {:?}", next);
                    }
                }
            }
        }
        tokens
    }
    fn eat(syms: &mut Vec<&str>) -> String {
        let mut next = syms.remove(0);
        let mut val = "".to_string();
        while !TokenTypes::WHITESPACE.tokens().regex().is_match(next) && !syms.is_empty() {
            val.push_str(next);
            next = syms.remove(0);
        }
        val.push_str(next);
        let trimmed = val.trim();
        trimmed.to_string()
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenTypes,
    pub data_type: DataTypes,
    pub value: String,
}