use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use reduce::Reduce;

use crate::token::data_types::DataTypes;
use crate::token::token_type::TokenTypes;
use crate::token::tokenizer::Token;

pub struct Evaluator {
    pub global_scope: HashMap<String, Reference>,
}

#[derive(Debug)]
pub enum Reference {
    String(String, bool),
    Boolean(bool, bool),
    Number(i32, bool),
}

#[derive(Debug, Copy, Clone)]
pub enum Operator<'a> {
    ADDITION(&'a str),
    SUBTRACTION(&'a str),
    MULTIPLICATION(&'a str),
    DIVISION(&'a str),
    CONCAT(&'a str),
    UNSUPPORTED(&'a str),
}

impl Evaluator {
    pub fn get_scope(&mut self) -> &HashMap<String, Reference, RandomState> {
        &self.global_scope
    }
    pub fn evaluate(&mut self, tree: &mut Vec<Vec<Token>>) {
        while !tree.is_empty() {
            let mut branch = tree.remove(0);
            Self::evaluate_branch(&mut branch, tree, &mut self.global_scope);
        }
    }
    pub fn evaluate_branch(
        branch: &mut Vec<Token>,
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
    ) {
        while !branch.is_empty() {
            let mut token = branch.remove(0);
            match token.token_type {
                TokenTypes::IDENTIFIER => Self::do_evaluate(&mut token, tree, scope),
                TokenTypes::PRINT => Self::do_print(tree, scope),
                _ => panic!("Unknown token type"),
            };
        }
    }
    pub fn do_print(tree: &mut Vec<Vec<Token>>, scope: &mut HashMap<String, Reference>) {
        let next_branch = tree.remove(0);
        let next_token = next_branch.get(0).unwrap();
        let next_token_value = next_token.value.as_str();
        let scope_value = scope.get(next_token_value);
        if scope_value.is_some() {
            println!("{:?}", scope_value.unwrap());
        } else {
            println!("{}", next_token_value);
        }
    }

    pub fn do_evaluate(
        token: &mut Token,
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
    ) {
        let left_token_value = token.value.as_str();
        let left_token_maybe_reference = scope.get(left_token_value);
        if left_token_maybe_reference.is_some() {
            let left_reference = left_token_maybe_reference.unwrap();
            if let Reference::String(_string_value, string_mutable) = left_reference {
                if string_mutable.eq(&true) {
                    Self::update_string_reference(token, tree, scope);
                    return;
                } else {
                    panic!("Can't update immutable string.")
                }
            }
            if let Reference::Number(_number_value, number_mutable) = left_reference {
                if number_mutable.eq(&true) {
                    Self::update_number_reference(token, tree, scope);
                    return;
                } else {
                    panic!("Can't update immutable number.")
                }
            }
            if let Reference::Boolean(_boolean_value, boolean_mutable) = left_reference {
                if boolean_mutable.eq(&true) {
                    Self::update_boolean_reference(token, tree, scope);
                    return;
                } else {
                    panic!("Can't update immutable boolean.")
                }
            }
        } else {
            match token.data_type {
                DataTypes::TypeMutableString => Self::create_string(tree, scope, true),
                DataTypes::TypeImmutableString => Self::create_string(tree, scope, false),
                DataTypes::TypeMutableNumber => Self::create_number(tree, scope, true),
                DataTypes::TypeImmutableNumber => Self::create_number(tree, scope, false),
                DataTypes::TypeMutableBoolean => Self::create_boolean(tree, scope, true),
                DataTypes::TypeImmutableBoolean => Self::create_boolean(tree, scope, false),
                _ => (),
            }
        }
    }
    pub fn create_string(
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
        mutable: bool,
    ) {
        let next_branch = tree.remove(0);
        let identifier_token = next_branch.get(0).unwrap();
        let identifier_token_value = &identifier_token.value;
        let val;
        if tree.len() > 0 {
            let value_token_branch = tree.remove(0);
            let value_token = value_token_branch.get(0);
            let value = &value_token.unwrap().value;
            let maybe_quote_pos = value.find("\"");
            if maybe_quote_pos.is_some() {
                val = value.clone();
            } else {
                let operator = Self::get_string_operator(&value_token.unwrap());
                if Self::all_names_in_scope(value_token.unwrap(), scope, operator) {
                    val = Self::string_concat(value_token.unwrap(), scope, &operator);
                } else {
                    let mut vec = Vec::new();
                    let tk = Token {
                        token_type: value_token.unwrap().token_type,
                        data_type: value_token.unwrap().data_type,
                        value: value_token.unwrap().value.clone(),
                    };
                    vec.push(tk);
                    tree.insert(0, vec);
                    val = String::new();
                }
            }
        } else {
            val = String::new();
        }
        let reference = Reference::String(val.replace("\"", ""), mutable);
        scope.insert(identifier_token_value.clone(), reference);
    }

    pub fn string_concat(
        token: &Token,
        scope: &mut HashMap<String, Reference>,
        operator: &Operator,
    ) -> String {
        let mut value = String::new();
        if let Operator::CONCAT(pattern) = operator {
            let references: Vec<String> = Referencer::get_all(token, pattern, scope);
            value = references.into_iter().reduce(|a, b| a + &b).unwrap();
        }
        value
    }

    pub fn create_number(
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
        mutable: bool,
    ) {
        let next_branch = tree.remove(0);
        let identifier_token = next_branch.get(0).unwrap();
        let identifier_token_value = &identifier_token.value;
        let val;
        if tree.len() > 0 {
            let value_token_branch = tree.remove(0);
            let value_token = value_token_branch.get(0);
            let parsed = value_token.unwrap().value.parse();
            if parsed.is_err() {
                let operator = Self::get_numeric_operator(value_token.unwrap());
                if Self::all_names_in_scope(value_token.unwrap(), scope, operator) {
                    val = Self::apply_numeric_operation(value_token.unwrap(), scope, &operator);
                } else {
                    let mut vec = Vec::new();
                    let tk = Token {
                        token_type: value_token.unwrap().token_type,
                        data_type: value_token.unwrap().data_type,
                        value: value_token.unwrap().value.clone(),
                    };
                    vec.push(tk);
                    tree.insert(0, vec);
                    val = 0;
                }
            } else {
                val = parsed.unwrap();
            }
        } else {
            val = 0;
        }
        let reference = Reference::Number(val, mutable);
        scope.insert(identifier_token_value.clone(), reference);
    }
    pub fn create_boolean(
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
        mutable: bool,
    ) {
        let next_branch = tree.remove(0);
        let identifier_token = next_branch.get(0).unwrap();
        let identifier_token_value = &identifier_token.value;
        let value_token_branch = tree.remove(0);
        let value_token = value_token_branch.get(0);
        println!("create boolean {}", identifier_token_value);
        let val;
        if value_token.is_some() {
            val = value_token.unwrap().value.parse().unwrap();
        } else {
            val = false;
        }
        let reference = Reference::Boolean(val, mutable);
        scope.insert(identifier_token_value.clone(), reference);
    }
    pub fn get_numeric_operator(token: &Token) -> Operator {
        if token.value.contains(".") {
            return Operator::ADDITION(".");
        };
        if token.value.contains("-") {
            return Operator::SUBTRACTION("-");
        };
        if token.value.contains("\\/") {
            return Operator::MULTIPLICATION("/");
        };
        if token.value.contains("\\") {
            return Operator::DIVISION("\\");
        };
        Operator::UNSUPPORTED("")
    }
    pub fn get_string_operator(token: &Token) -> Operator {
        if token.value.contains(".") {
            return Operator::CONCAT(".");
        };
        Operator::UNSUPPORTED("")
    }
    pub fn all_names_in_scope(
        token: &Token,
        scope: &mut HashMap<String, Reference>,
        operator: Operator,
    ) -> bool {
        if let Operator::ADDITION(pattern) = operator {
            return Self::check_names(token, pattern, scope);
        }
        if let Operator::SUBTRACTION(pattern) = operator {
            return Self::check_names(token, pattern, scope);
        }
        if let Operator::MULTIPLICATION(pattern) = operator {
            return Self::check_names(token, pattern, scope);
        }
        if let Operator::DIVISION(pattern) = operator {
            return Self::check_names(token, pattern, scope);
        }
        if let Operator::CONCAT(pattern) = operator {
            return Self::check_names(token, pattern, scope);
        }
        return false;
    }
    pub fn check_names(
        token: &Token,
        pattern: &str,
        scope: &mut HashMap<String, Reference>,
    ) -> bool {
        let names: Vec<&str> = token.value.split(pattern).collect();
        let count = scope.keys().len() as i32;
        let mut found = 0;
        for name in names.iter() {
            let reference = scope.get(name.to_owned());
            if reference.is_some() {
                found = found + 1;
            }
        }
        if found.eq(&count) {
            return true;
        }
        return false;
    }
    pub fn apply_numeric_operation(
        token: &Token,
        scope: &mut HashMap<String, Reference>,
        operator: &Operator,
    ) -> i32 {
        let mut value = 0;
        if let Operator::ADDITION(pattern) = operator {
            let references: Vec<i32> = Referencer::get_all(token, pattern, scope);
            value = references.into_iter().reduce(|a, b| a + b).unwrap();
        }
        if let Operator::SUBTRACTION(pattern) = operator {
            let references: Vec<i32> = Referencer::get_all(token, pattern, scope);
            value = references.into_iter().reduce(|a, b| a - b).unwrap();
        }
        if let Operator::MULTIPLICATION(pattern) = operator {
            let references: Vec<i32> = Referencer::get_all(token, pattern, scope);
            value = references.into_iter().reduce(|a, b| a * b).unwrap();
        }
        if let Operator::DIVISION(pattern) = operator {
            let references: Vec<i32> = Referencer::get_all(token, pattern, scope);
            value = references.into_iter().reduce(|a, b| a / b).unwrap();
        }
        return value;
    }
    pub fn update_string_reference(
        token: &Token,
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
    ) {
        let right_branch = tree.remove(0);
        let right_identifier_token = right_branch.get(0).unwrap();
        let maybe_right_value = scope.get(right_identifier_token.value.clone().as_str());
        if maybe_right_value.is_some() {
            let right_reference = maybe_right_value.unwrap();
            if let Reference::String(right_value, _right_mutable) = right_reference {
                scope.insert(
                    token.value.clone(),
                    Reference::String(right_value.clone(), true),
                );
            }
        } else {
            // if (allNamesInScope(next, scope, '.')) {
            //     scope[branch.value].value = stringConcat(next, scope);
            // } else {
            //     //scope[branch.value].value = next.value.replace(/\"/g, '');;
            // }
        }
    }
    pub fn update_number_reference(
        token: &Token,
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
    ) {
    }
    pub fn update_boolean_reference(
        token: &Token,
        tree: &mut Vec<Vec<Token>>,
        scope: &mut HashMap<String, Reference>,
    ) {
    }
}

pub struct Referencer {}

pub trait GetAll<T> {
    fn get_all(token: &Token, pattern: &&str, scope: &mut HashMap<String, Reference>) -> Vec<T>;
}

impl GetAll<String> for Referencer {
    fn get_all(
        token: &Token,
        pattern: &&str,
        scope: &mut HashMap<String, Reference>,
    ) -> Vec<String> {
        let names: Vec<&str> = token.value.split(pattern).collect();
        return names
            .into_iter()
            .map(|name| {
                let maybe_ref = scope.get(name.clone());
                if maybe_ref.is_some() {
                    let r = maybe_ref.unwrap();
                    if let Reference::String(val, _mutable) = r {
                        return val.to_owned();
                    }
                }
                return String::from("");
            })
            .collect();
    }
}

impl GetAll<i32> for Referencer {
    //noinspection DuplicatedCode
    fn get_all(token: &Token, pattern: &&str, scope: &mut HashMap<String, Reference>) -> Vec<i32> {
        let names: Vec<&str> = token.value.split(pattern).collect();
        return names
            .into_iter()
            .map(|name| {
                let maybe_ref = scope.get(name.clone());
                if maybe_ref.is_some() {
                    let r = maybe_ref.unwrap();
                    if let Reference::Number(val, _mutable) = r {
                        return val.to_owned();
                    }
                }
                return 0;
            })
            .collect();
    }
}

// }

// function updateStringReference(instance, branch, tree, scope) {
//     var next = tree.shift();
//     if (instance.mutable) {
//         if (scope[next.value] !== undefined) {
//             //update from reference
//             scope[branch.value].value = scope[next.value].value;
//         } else {
//             if (allNamesInScope(next, scope, '.')) {
//                 scope[branch.value].value = stringConcat(next, scope);
//             } else {
//                 scope[branch.value].value = next.value.replace(/\"/g, '');;
//             }
//         }
//     } else {
//         throw new Error('cant re-assign immutable string');
//     }
// }

// function updateNumberReference(instance, branch, tree, scope) {
//     console.log('tree ', tree);
//     var next = tree.shift();
//     console.log('tree ', tree);
//     if (instance.mutable) {
//         if (scope[next.value] !== undefined) {
//             console.log('update from reference ', next.value);
//             //update from reference
//             scope[branch.value].value = scope[next.value].value;
//         } else {
//             console.log('get operator update ', instance);
//             var operator = getMemberOperator(next);
//             console.log('get operator found ', operator);
//             if (allNamesInScope(next, scope, operator)) {
//                 scope[branch.value].value = applyNumberOperation(next, scope, operator);
//             } else {
//                 if (operator) {
//                     scope[branch.value].value = applyNumberOperation(next, scope, operator);
//                 } else {
//                     scope[branch.value].value = parseInt(next.value);
//                 }
//             }
//         }
//     } else {
//         throw new Error('cant re-assign immutable number');
//     }
// }
