#![allow(unused_imports, unused_variables, dead_code)]

pub mod lexer {
    use crate::tokens::token::Token;
    use std::collections::HashMap;
    pub struct Lexer;

    impl Lexer {
        pub fn new() -> Self {
            Self
        }

        pub fn lex<'a>(self: &'a Self, s: &'a str) -> Vec<HashMap<Token, &str>> {
            let split = s.split(" ");
            let mut splits = vec![];
            for i in split {
                splits.push(HashMap::from([(Token::new(i), i)]));
            }
            splits
        }
    }
}
