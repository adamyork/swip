use std::collections::HashMap;

use crate::parser::evaluator::{Evaluator, Reference};
use crate::parser::token_parser::TokenParser;
use crate::token::tokenizer::Token;
use crate::token::tokenizer::Tokenizer;

#[test]
fn test_single_mutable_number_no_value() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**$ number1");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("number1").unwrap();
    if let Reference::Number(number_val, number_mutable) = value {
        assert_eq!(number_val, &0);
        assert_eq!(number_mutable, &true);
    } else {
        panic!("Reference is not a number.")
    }
}

fn test_single_mutable_number() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**$ number1 32");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("number1").unwrap();
    if let Reference::Number(number_val, number_mutable) = value {
        assert_eq!(number_val, &32);
        assert_eq!(number_mutable, &true);
    } else {
        panic!("Reference is not a number.")
    }
}

fn test_single_immutable_number() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("*$ number1 32");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("number1").unwrap();
    if let Reference::Number(number_val, number_mutable) = value {
        assert_eq!(number_val, &32);
        assert_eq!(number_mutable, &false);
    } else {
        panic!("Reference is not a number.")
    }
}

fn test_multi_mixed_number() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("*$ number1 32\n**$ number 2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("number1").unwrap().clone();
    if let Reference::Number(number_val, number_mutable) = value1 {
        assert_eq!(number_val, &32);
        assert_eq!(number_mutable, &false);
    } else {
        panic!("Reference is not a number.")
    }
    let value2 = evaluator.get_scope().get("number2").unwrap().clone();
    if let Reference::Number(number_val, number_mutable) = value2 {
        assert_eq!(number_val, &0);
        assert_eq!(number_mutable, &true);
    } else {
        panic!("Reference is not a number.")
    }
}

#[test]
fn test_single_mutable_string_no_value() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("string1").unwrap();
    if let Reference::String(str_val, str_mutable) = value {
        assert_eq!(str_val, "");
        assert_eq!(str_mutable, &true);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_single_mutable_string() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1 \"hello\"");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("string1").unwrap();
    if let Reference::String(string_val, string_mutable) = value {
        assert_eq!(string_val, "hello");
        assert_eq!(string_mutable, &true);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_single_immutable_string() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("*\" string1 \"hello\"");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("string1").unwrap();
    if let Reference::String(string_val, string_mutable) = value {
        assert_eq!(string_val, "hello");
        assert_eq!(string_mutable, &false);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_multi_mixed_string() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("*\" string1 \"hello\"\n**\" string2 \"world\"");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("string1").unwrap().clone();
    if let Reference::String(string_val, string_mutable) = value1 {
        assert_eq!(string_val, "hello");
        assert_eq!(string_mutable, &false);
    } else {
        panic!("Reference is not a string.")
    }
    let value2 = evaluator.get_scope().get("string2").unwrap().clone();
    if let Reference::String(string_val, string_mutable) = value2 {
        assert_eq!(string_val, "world");
        assert_eq!(string_mutable, &true);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_update_string_with_raw() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**\" string1 \"hello\"\nstring1 \"world\"");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("string1").unwrap().clone();
    if let Reference::String(string_val, string_mutable) = value1 {
        assert_eq!(string_val, "world");
        assert_eq!(string_mutable, &true);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_update_string_with_var() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**\" string1 \"hello\"\n**\" string2 \"world\"\nstring1 string2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("string1").unwrap().clone();
    if let Reference::String(string_val, string_mutable) = value1 {
        assert_eq!(string_val, "world");
        assert_eq!(string_mutable, &true);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_update_string_with_concat() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer
        .tokenize("**\" string1 \"hello\"\n**\" string2 \"world\"\nstring1 string1.string2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("string1").unwrap().clone();
    if let Reference::String(string_val, string_mutable) = value1 {
        assert_eq!(string_val, "helloworld");
        assert_eq!(string_mutable, &true);
    } else {
        panic!("Reference is not a string.")
    }
}

#[test]
fn test_single_mutable_boolean_no_value() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**b boolean1");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("boolean1").unwrap();
    if let Reference::Boolean(bool_val, bool_mutable) = value {
        assert_eq!(bool_val, &false);
        assert_eq!(bool_mutable, &true);
    } else {
        panic!("Reference is not a boolean.")
    }
}

#[test]
fn test_single_mutable_boolean() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**b boolean1 true");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("boolean1").unwrap();
    if let Reference::Boolean(bool_val, bool_mutable) = value {
        assert_eq!(bool_val, &true);
        assert_eq!(bool_mutable, &true);
    } else {
        panic!("Reference is not a boolean.")
    }
}

#[test]
fn test_single_immutable_boolean() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("*b boolean1 true");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value = evaluator.get_scope().get("boolean1").unwrap();
    if let Reference::Boolean(bool_val, bool_mutable) = value {
        assert_eq!(bool_val, &true);
        assert_eq!(bool_mutable, &false);
    } else {
        panic!("Reference is not a boolean.")
    }
}

#[test]
#[should_panic]
fn test_immutable_cant_update() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer
        .tokenize("*\" string1 \"hello\"\n**\" string2 \"world\"\nstring1 string1.string2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
}

#[test]
fn test_print_var() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> = tokenizer.tokenize("**b someBool true\n# someBool");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
}

#[test]
fn test_update_boolean_with_and() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**b boolean1 true\n**b boolean2 false\nboolean1 boolean1.boolean2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("boolean1").unwrap().clone();
    if let Reference::Boolean(bool_val, bool_mutable) = value1 {
        assert_eq!(bool_val, &false);
        assert_eq!(bool_mutable, &true);
    } else {
        panic!("Reference is not a boolean.")
    }
}

#[test]
fn test_update_boolean_with_or() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**b boolean1 true\n**b boolean2 false\nboolean1 boolean1\\boolean2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("boolean1").unwrap().clone();
    if let Reference::Boolean(bool_val, bool_mutable) = value1 {
        assert_eq!(bool_val, &true);
        assert_eq!(bool_mutable, &true);
    } else {
        panic!("Reference is not a boolean.")
    }
}

#[test]
fn test_update_mutable_number_with_raw() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**$ number1 32\nnumber1 100");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("number1").unwrap().clone();
    if let Reference::Number(num_val, num_mutable) = value1 {
        assert_eq!(num_val, &100);
        assert_eq!(num_mutable, &true);
    } else {
        panic!("Reference is not a number.")
    }
}

#[test]
fn test_update_mutable_number_with_var() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**$ number1 32\n**$ number2 45\nnumber1 number2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("number1").unwrap().clone();
    if let Reference::Number(num_val, num_mutable) = value1 {
        assert_eq!(num_val, &45);
        assert_eq!(num_mutable, &true);
    } else {
        panic!("Reference is not a number.")
    }
}

#[test]
fn test_update_mutable_number_with_sum() {
    let tokenizer = Tokenizer { debug: false };
    let parser = TokenParser { debug: false };
    let mut tokens: Vec<Token> =
        tokenizer.tokenize("**$ number1 32\n**$ number2 45\nnumber1 number1.number2");
    let mut tree: Vec<Vec<Token>> = parser.parse(&mut tokens);
    let gs: HashMap<String, Reference> = HashMap::new();
    let mut evaluator = Evaluator {
        global_scope: gs,
        debug: false,
    };
    evaluator.evaluate(&mut tree);
    let value1 = evaluator.get_scope().get("number1").unwrap().clone();
    if let Reference::Number(num_val, num_mutable) = value1 {
        assert_eq!(num_val, &77);
        assert_eq!(num_mutable, &true);
    } else {
        panic!("Reference is not a number.")
    }
}

