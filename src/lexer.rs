#![allow(unused_imports, unused_variables, dead_code, unused_assignments)]

pub mod lexer {
    use crate::error::error::{Error, ErrorType};
    use crate::tokens::token::Token;
    use crate::types::*;
    use crate::Regex;
    use std::collections::btree_map::Values;
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

            if Token::new(s) == Token::NEWLINE {
                splits.push(HashMap::from([(Token::NEWLINE, "\n")]));
            } else if Token::new(s.chars().nth(0).unwrap().to_string().as_str()) == Token::STRING {
                let c = self.consume_until(&s[1..], '"', false, linenumber);
                splits.push(HashMap::from([(Token::STRING, c)]))
            } else if s.chars().nth(0).unwrap().to_string().as_str() == "f"
                && Token::new(s.chars().nth(1).unwrap().to_string().as_str()) == Token::STRING
            {
                let index = match Self::find_char(&self, s, '{') {
                    Some(e) => e,
                    None => -1,
                };
                let fvalue = if index != -1 {
                    self.consume_until(&s[index as usize + 1..], '}', false, linenumber)
                } else {
                    Error::new(
                        ErrorType::TypeError,
                        format!("Unneeded f string"),
                        linenumber,
                    );
                    unreachable!("shouldnt")
                };
                let stc = self.consume_until(&s[index as usize..], '}', true, linenumber);
                println!("{fvalue}");
                splits.push(HashMap::from([
                    (Token::FSTRING, s),
                    (Token::VARNAME, fvalue),
                    (Token::OTHER, stc),
                ]))
            } else {
                let split: Vec<&str> = s.split(" ").collect();
                let f = split[0];
                if Token::new(f) == Token::IDENTIFIER {
                    let varname = split[1];
                    if Token::new(split[2]) == Token::ASSIGNMENT {
                        let varvalue = if split[3] != "" {
                            split[3]
                        } else {
                            Error::new(ErrorType::TokenError, format!("Missing value"), linenumber);
                            unreachable!("program should end when error produces")
                        };
                        let t: &str;
                        if varvalue.chars().nth(0).unwrap() == '"' {
                            t = "string";
                        } else {
                            if varvalue.contains(".") {
                                t = "float";
                            } else {
                                t = "integer";
                            }
                        }

                        let constructed = HashMap::from([
                            (Token::IDENTIFIER, "let"),
                            (Token::TYPE, t),
                            (Token::VARNAME, varname),
                            (Token::VARVALUE, varvalue),
                        ]);
                        splits.push(constructed);
                    } else {
                        Error::new(ErrorType::TokenError, format!("Missing \"=\""), linenumber);
                        unreachable!("program should end when error produces")
                    };
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
                unreachable!("program should end when error produces")
            }
        }

        pub fn find_char(self: &Self, s: &str, find: char) -> Option<i32> {
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
                println!("{find} not found");
                None
            }
        }

        //pub fn consume_until_lines;
    }
}
