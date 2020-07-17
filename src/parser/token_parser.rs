use crate::token::token_type::TokenTypes;
use crate::token::tokenizer::Token;

pub struct TokenParser {}

impl TokenParser {
    pub fn parse(&self, tokens: &mut Vec<Token>) -> Vec<Vec<Token>> {
        let mut tree: Vec<Vec<Token>> = Vec::new();
        while !tokens.is_empty() {
            let token: Token = tokens.remove(0);
            if token.token_type == TokenTypes::LPAREN {
                let branch: Vec<Token> = Self::eat(tokens);
                tree.push(branch);
            }
            let token_vector: Vec<Token> = vec![token];
            tree.push(token_vector);
        }
        tree
    }
    pub fn eat(tokens: &mut Vec<Token>) -> Vec<Token> {
        let mut token: Token = tokens.remove(0);
        let mut branch: Vec<Token> = Vec::new();
        while token.token_type != TokenTypes::RPAREN && !tokens.is_empty() {
            branch.push(token);
            token = tokens.remove(0);
        }
        return branch;
    }
}
