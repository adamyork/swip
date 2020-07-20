use crate::parser::token_parser::TokenParser;
use crate::token::tokenizer::Token;
use crate::token::tokenizer::Tokenizer;

#[test]
fn test_no_branch_tree() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**$ number1 27\n**$ number2 42");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    assert_eq!(tree.len(), 6);
    assert_eq!(tree.get(0).unwrap().len(), 1);
    assert_eq!(tree.get(1).unwrap().len(), 1);
    assert_eq!(tree.get(2).unwrap().len(), 1);
    assert_eq!(tree.get(3).unwrap().len(), 1);
    assert_eq!(tree.get(4).unwrap().len(), 1);
    assert_eq!(tree.get(5).unwrap().len(), 1);
}
