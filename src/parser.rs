#![allow(unused_imports, unused_variables, dead_code)]

pub mod parser {
    use crate::lexer::lexer::Lexer;
    use crate::tokens::token::Token;
    use std::collections::HashMap;
    use std::iter::Enumerate;
    pub struct Parser;

    impl Parser {
        pub fn new() -> Self {
            Self
        }

        pub fn parse(self: Self, s: &str) {
            let lexer = Lexer::new();
            let mut linenum = 0;
            for i in s.lines() {
                let res = lexer.lex(i, linenum);
                println!("{:?}", res);
                self.eval(&res);
                linenum += 1;
            }
        }

        fn eval(self: &Self, s: &Vec<HashMap<Token, &str>>) {
            for i in s {
                for (i, k) in i.iter() {
                    if i == &Token::STRING {
                        println!(r#"{}"#, k);
                    }
                }
            }
        }
    }
}
