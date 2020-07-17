use std::collections::HashMap;

use swip::parser::evaluator::{Evaluator, Reference};
use swip::parser::token_parser::TokenParser;
use swip::token::token_type::TokenTypes;
use swip::token::tokenizer::Token;
use swip::token::tokenizer::Tokenizer;

fn main() {
    println!("{}", TokenTypes::WHITESPACE.tokens().regex().is_match(" "));
    println!("{}", TokenTypes::NUMBER.tokens().regex().is_match("1"));
    println!(
        "{}",
        TokenTypes::IDENTIFIER.tokens().regex().is_match("sdf")
    );
    println!("{}", TokenTypes::LPAREN.tokens().regex().is_match("("));
    println!("{}", TokenTypes::RPAREN.tokens().regex().is_match(")"));
    println!("{}", TokenTypes::PRINT.tokens().regex().is_match("#"));
    let tokenizer = Tokenizer {};
    let parser = TokenParser {};
    //let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1 \"hey\"");
    //let mut tokens: Vec<Token> = tokenizer.tokenize("# \"hi\"");
    //let mut tokens: Vec<Token> = tokenizer.tokenize("**b boolean1 false");
    //let mut tokens: Vec<Token> = tokenizer.tokenize("**$ number1 27\n**$ number2 45\n**$ number3 number1.number2");
    //let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1");
    //let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1 \"hi\"");
    //let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1 \"hi\"\n**\" string2 \"there\"");
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**\" string1 \"hi\"\n**\" string2 \"there\"\nstring1 string2");
    for t in &tokens {
        println!("{:?}", t.token_type);
    }
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    // for v in tree {
    //     for i in v {
    //         println!("{:?}", &i);
    //     }
    // }
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator { global_scope: gs };
    evaluator.evaluate(&mut tree);
    println!("result {:?}", evaluator.get_scope().get("string1"));
    println!("result {:?}", evaluator.get_scope().get("string2"));
}
