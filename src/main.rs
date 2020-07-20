use std::collections::HashMap;
use std::env;
use std::io;
use std::process::exit;

use swip::parser::evaluator::{Evaluator, Reference};
use swip::parser::token_parser::TokenParser;
use swip::token::token_type::TokenTypes;
use swip::token::tokenizer::Token;
use swip::token::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;
    if args.len() > 1 {
        if args.contains(&String::from("-d")) {
            debug = true;
        }
    }
    let tokenizer = Tokenizer {
        debug
    };
    let parser = TokenParser {
        debug
    };
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug,
    };
    let mut repl = true;
    if debug {
        println!("Starting REPL");
    }
    while repl {
        repl = do_repl(&tokenizer, &parser, &mut evaluator);
    }
}

fn do_repl(tokenizer: &Tokenizer, parser: &TokenParser, evaluator: &mut Evaluator) -> bool {
    println!("swip >");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let input_str = input.as_str();
            if input_str.eq("exit\n") {
                exit(0);
            }
            let mut tokens: Vec<Token> = tokenizer.tokenize(input_str);
            let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
            evaluator.evaluate(&mut tree);
            return true;
        }
        Err(error) => println!("error: {}", error),
    }
    return false;
}
