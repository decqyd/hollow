#![allow(unused_imports, unused_variables, dead_code)]

pub mod lexer {
    use crate::tokens::token::Token;
    use crate::types::Str;
    use std::collections::HashMap;
    use std::ptr::null;
    pub struct Lexer;

    impl Lexer {
        pub fn new() -> Self {
            Self
        }

        pub fn lex<'a>(self: &'a Self, s: &'a str) -> Vec<HashMap<Token, &str>> {
            let mut splits = vec![];

            if s == "" || s == "\n" {
                splits.push(HashMap::from([(Token::NEWLINE, "\n")]));
            } else if s.chars().nth(0).unwrap() == '"' {
                let c = self.consume_until(&s[1..], '"', false);
                splits.push(HashMap::from([(Token::STRING, c)]))
            } else {
                let split: Vec<&str> = s.split(" ").collect();
                if split[0] == "let" {
                    let varname = split[1];
                    let equals = split[2];
                    let varvalue = split[3];
                    splits.push(HashMap::from([
                        (Token::new("let"), "let"),
                        (Token::VARNAME, varname),
                        (Token::ASSIGNMENT, "="),
                        (Token::VARVALUE, varvalue),
                    ]));
                }
            }

            splits
        }

        pub fn consume_until<'a>(self: &Self, s: &'a str, until: char, inclusive: bool) -> &'a str {
            let mut i = 0;
            let mut flag = false;
            for char in s.chars() {
                if char != until {
                    i += 1
                } else {
                    flag = true;
                    break;
                }
            }
            if flag {
                if !inclusive {
                    &s[..i]
                } else {
                    &s[..=i]
                }
            } else {
                panic!("missing {until}");
            }
        }
    }
}
