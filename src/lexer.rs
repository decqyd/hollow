#![allow(unused_variables, dead_code, unused_assignments)]

use crate::error::{Error, ErrorType};
use crate::tokens::Token;
use crate::types::*;
use std::collections::HashMap;

pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self
    }

    pub fn lex<'a>(&'a self, s: &'a str, linenumber: i32) -> Vec<HashMap<Token, &str>> {
        let mut splits = vec![];

        if Token::new(s) == Token::NEWLINE {
            splits.push(HashMap::from([(Token::NEWLINE, "\n")]));
        } else if Token::new(s.chars().next().unwrap().to_string().as_str()) == Token::STRING {
            self.check_last(s, '"', linenumber);
            let c = self.consume_until(&s[1..], '"', false, linenumber);
            splits.push(HashMap::from([(Token::STRING, c)]));
        } else if s.chars().next().unwrap().to_string().as_str() == "f"
            && Token::new(s.chars().nth(1).unwrap().to_string().as_str()) == Token::STRING
        {
            // TODO: add support for multiple variables

            let mut count = 0;

            let index = Self::find_char(self, s, '{').unwrap_or(-1);
            if index != -1 {
                splits.push(HashMap::from([(Token::FSTRING, s)]))
            }
            count += 1;

            if count < 1 {
                Error::new(
                    ErrorType::TypeError,
                    "No variables passed to f string".to_string(),
                    linenumber,
                );
                unreachable!("shouldnt")
            }
        } else {
            let split: Vec<&str> = s.split(' ').collect();
            let f = split[0];
            // if token is let
            if Token::new(f) == Token::IDENTIFIER {
                let varname: &str = split[1];
                if Token::new(split[2]) == Token::ASSIGNMENT {
                    let a = match self.find_char(s, '=') {
                        Some(e) => e,
                        None => unreachable!("nerd"),
                    };

                    let slice = &s[a as usize + 2..];

                    let mut varvalue = if !split[3].is_empty() {
                        slice
                    } else {
                        Error::new(
                            ErrorType::TokenError,
                            "Missing value".to_string(),
                            linenumber,
                        );
                        unreachable!("program should end when error produces")
                    };

                    let t: &str;
                    if varvalue.starts_with('"') {
                        t = "string";
                        varvalue = self.consume_until(&varvalue[1..], '"', false, linenumber);
                    } else if varvalue.contains('.') {
                        t = "float";
                        let varvalue: f32 = match varvalue.parse() {
                            Ok(e) => e,
                            Err(e) => {
                                Error::new(
                                    ErrorType::TypeError,
                                    format!("{varvalue} is not an float"),
                                    linenumber,
                                );
                                unreachable!("ddodood")
                            }
                        };
                    } else {
                        t = "integer";
                        let varvalue: i128 = match varvalue.parse() {
                            Ok(e) => e,
                            Err(e) => {
                                Error::new(
                                    ErrorType::TypeError,
                                    format!("{varvalue} is not an int"),
                                    linenumber,
                                );
                                unreachable!("ddodood")
                            }
                        };
                    }

                    let constructed = HashMap::from([
                        (Token::IDENTIFIER, "let"),
                        (Token::TYPE, t),
                        (Token::VARNAME, varname),
                        (Token::VARVALUE, varvalue),
                    ]);
                    splits.push(constructed);
                } else {
                    Error::new(
                        ErrorType::TokenError,
                        "Missing \"=\"".to_string(),
                        linenumber,
                    );
                    unreachable!("program should end when error produces")
                };
            } else if Token::new(f) == Token::FUNCTIONIDENT {
                let funcname = split[1];
            } else {
                Error::new(
                    ErrorType::TokenError,
                    format!("Unexpected token \"{f}\""),
                    linenumber,
                )
            }
        }

        splits
    }

    pub fn consume_until<'a>(
        &self,
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
            Error::new(
                ErrorType::TokenError,
                format!("Missing {until}"),
                linenumber,
            );
            unreachable!("program should end when error produces")
        }
    }

    pub fn find_char(&self, s: &str, find: char) -> Option<i32> {
        // TODO: make it so this fucntion can find multiple chars

        let mut i = 0;
        let mut flag = false;

        for char in s.chars() {
            if char != find {
                i += 1
            } else {
                flag = true;
                break;
            }
        }

        if flag {
            Some(i)
        } else {
            None
        }
    }

    pub fn check_last(&self, s: &str, is: char, linenumber: i32) {
        let last = s.chars().last().unwrap();
        if last != is {
            Error::new(
                ErrorType::TokenError,
                "Unexpected token".to_string(),
                linenumber,
            );
        }
    }

    //pub fn consume_until_lines;
}
