#![allow(unused_imports, unused_variables, dead_code, unused_assignments)]

use crate::error::{Error, ErrorType};
use crate::tokens::Token;
use crate::types::*;
use crate::Regex;
use std::collections::btree_map::Values;
use std::collections::HashMap;
use std::fmt::format;
use std::ptr::null;
pub struct Lexer;

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

impl Lexer {
    pub fn new() -> Self {
        Self
    }

    pub fn lex<'a>(&'a self, s: &'a str, linenumber: i32) -> Vec<HashMap<Token, &str>> {
        let mut splits = vec![];

        if Token::new(s) == Token::NEWLINE {
            splits.push(HashMap::from([(Token::NEWLINE, "\n")]));
        } else if Token::new(s.chars().next().unwrap().to_string().as_str()) == Token::STRING {
            let c = self.consume_until(&s[1..], '"', false, linenumber);
            splits.push(HashMap::from([(Token::STRING, c)]))
        } else if s.chars().next().unwrap().to_string().as_str() == "f"
            && Token::new(s.chars().nth(1).unwrap().to_string().as_str()) == Token::STRING
        {
            // TODO: add support for multiple variables
            let index = Self::find_char(self, s, '{').unwrap_or(-1);

            let fvalue = if index != -1 {
                self.consume_until(&s[index as usize + 1..], '}', false, linenumber)
            } else {
                Error::new(
                    ErrorType::TypeError,
                    "Unneeded f string".to_string(),
                    linenumber,
                );
                unreachable!("shouldnt")
            };

            let stc = self.consume_until(&s[index as usize..], '}', true, linenumber);
            splits.push(HashMap::from([
                (Token::FSTRING, s),
                (Token::VARNAME, fvalue),
                (Token::OTHER, stc),
            ]))
        } else {
            let split: Vec<&str> = s.split(' ').collect();
            let f = split[0];

            // if token is let
            if Token::new(f) == Token::IDENTIFIER {
                let varname = split[1];
                if Token::new(split[2]) == Token::ASSIGNMENT {
                    let varvalue = if !split[3].is_empty() {
                        split[3]
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
                    } else if varvalue.contains('.') {
                        t = "float";
                    } else {
                        t = "integer";
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
            } else if f == "proc" {
                let funcname = split[1];
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
        //let mut count = 0;
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
            println!("{find} not found");
            None
        }
    }

    //pub fn consume_until_lines;
}
