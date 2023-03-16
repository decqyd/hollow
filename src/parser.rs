#![allow(unused_imports, unused_variables, dead_code)]

pub mod parser {
    use crate::lexer::lexer::Lexer;
    use std::collections::HashMap;
    pub struct Parser;

    impl Parser {
        pub fn new() -> Self {
            Self
        }

        pub fn parse(self: Self, s: &str) {
            let lexer = Lexer::new();
            let mut lines = vec![];
            let mut list = vec![];
            for i in s.lines() {
                lines.push(i);
            }
            for i in &lines {
                let res = lexer.lex(i);
                list.push(res);
                println!("{}", lines[0]);
                println!("{:?}", list[0])
                //
            }
        }

        fn eval() {}
    }
}
