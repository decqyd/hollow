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
                        let stc = h[&Token::OTHER];
                        let fvalue = match vars.get(&h[&Token::VARNAME]) {
                            Some(e) => e,
                            None => {
                                Error::new(
                                    ErrorType::TokenError,
                                    format!("No variable named \"{}\"", h[&Token::VARNAME]),
                                    linenum,
                                );
                                unreachable!("gurgle")
                            }
                        };

                        let formatted = fstring.replace(stc, fvalue);
                        let stp =
                            Lexer::consume_until(&Lexer, &formatted[2..], '"', false, linenum);
                        println!(r#"{}"#, stp);
                    }
                }
            }
        }
    }
}
