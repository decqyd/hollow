#![allow(unused_imports, unused_variables, dead_code)]
use crate::error::{Error, ErrorType};
use crate::lexer::Lexer;
use crate::tokens::Token;
use std::collections::HashMap;
use std::iter::Enumerate;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(self, s: &str) {
        let mut vars: HashMap<&str, &str> = HashMap::new();
        let lexer = Lexer::new();

        for (linenum, i) in s.lines().enumerate() {
            let linenum = linenum as i32;
            let res = lexer.lex(i, linenum);

            for h in &res {
                for (i, k) in h.iter() {
                    if i == &Token::STRING {
                        println!(r"{}", k);
                    } else if i == &Token::IDENTIFIER {
                        vars.insert(h[&Token::VARNAME], h[&Token::VARVALUE]);
                    } else if i == &Token::FSTRING {
                        let fstring = h[&Token::FSTRING];
                        let mut result = String::new();
                        let mut start = 0;

                        while let Some(start_index) = fstring[start..].find('{') {
                            let end_index = fstring[start + start_index + 1..]
                                .find('}')
                                .map(|i| i + start + start_index + 1)
                                .unwrap_or_else(|| {
                                    Error::new(
                                        ErrorType::TokenError,
                                        "Unterminated fstring".to_owned(),
                                        linenum,
                                    );
                                    unreachable!("L")
                                });
                            result += &fstring[start..start + start_index];
                            let fvalue = lexer.consume_until(
                                &fstring[start + start_index + 1..],
                                '}',
                                false,
                                linenum,
                            );
                            let val = match vars.get(fvalue) {
                                Some(e) => e,
                                None => {
                                    Error::new(
                                        ErrorType::TokenError,
                                        format!("No variable named \"{}\"", fvalue),
                                        linenum,
                                    );
                                    unreachable!("gurgle")
                                }
                            };
                            result += val;
                            start = end_index + 1;
                        }
                        result += &fstring[start..];
                        result = lexer
                            .consume_until(&result[2..], '"', false, linenum)
                            .to_string();
                        println!("{}", result);
                    }
                }
                /*  let index = lexer.find_char(k, '{').unwrap_or(-1);

                    let fstring = h[&Token::FSTRING];
                    let stc = lexer.consume_until(&k[index as usize..], '}', true, linenum);
                    let fvalue =
                        lexer.consume_until(&k[index as usize + 1..], '}', false, linenum);

                    let val = match vars.get(&fvalue) {
                        Some(e) => e,
                        None => {
                            Error::new(
                                ErrorType::TokenError,
                                format!("No variable named \"{}\"", fvalue),
                                linenum,
                            );
                            unreachable!("gurgle")
                        }
                    };

                    let formatted = fstring.replace(stc, val);
                    let stp =
                        Lexer::consume_until(&Lexer, &formatted[2..], '"', false, linenum);

                    println!(r#"{}"#, stp);
                } */
            }
        }
    }
}
