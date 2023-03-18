#![allow(unused_imports, unused_variables, dead_code)]

pub mod lexer {
    use crate::error::error::{Error, ErrorType};
    use crate::tokens::token::Token;
    use crate::types::Str;
    use std::collections::HashMap;
    use std::fmt::format;
    use std::ptr::null;
    pub struct Lexer;

    impl Lexer {
        pub fn new() -> Self {
            Self
        }

        pub fn lex<'a>(self: &'a Self, s: &'a str, linenumber: i32) -> Vec<HashMap<Token, &str>> {
            let mut splits = vec![];

            if s == "" || s == "\n" {
                splits.push(HashMap::from([(Token::NEWLINE, "\n")]));
            } else if s.chars().nth(0).unwrap() == '"' {
                let c = self.consume_until(&s[1..], '"', false, linenumber);
                splits.push(HashMap::from([(Token::STRING, c)]))
            } else {
                let split: Vec<&str> = s.split(" ").collect();
                let f = split[0];
                if f == "let" {
                    let varname = split[1];
                    let equals = if split[2] == "=" {
                        split[2]
                    } else {
                        Error::new(ErrorType::TokenError, format!("Missing \"=\""), linenumber);
                        ""
                    };

                    let varvalue = split[3];
                    let constructed = HashMap::from([
                        (Token::new("let"), "let"),
                        (Token::VARNAME, varname),
                        (Token::ASSIGNMENT, equals),
                        (Token::VARVALUE, varvalue),
                    ]);
                    splits.push(constructed);
                } else if f == "proc" {
                    let funcname = split[1];
                }
            }

            splits
        }

        pub fn consume_until<'a>(
            self: &Self,
            s: &'a str,
            until: char,
            inclusive: bool,
            linenumber: i32,
        ) -> &'a str {
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
                let error = Error::new(
                    ErrorType::TokenError,
                    format!("Missing {until}"),
                    linenumber,
                );
                "Error"
            }
        }

        //pub fn consume_until_lines;
    }
}
