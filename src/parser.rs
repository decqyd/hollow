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

        pub fn eval(self: Self, s: &str) {
            let mut vars: HashMap<&str, &str> = HashMap::new();
            let lexer = Lexer::new();
            let mut linenum = 0;
            for i in s.lines() {
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

                            let fvalue = h[&Token::VARNAME];

                            let getvalue = match vars.get(&fvalue) {
                                Some(e) => e,
                                None => "dooodo",
                            };
                            let formatted = fstring.replace(&stc, getvalue);
                            let stp =
                                Lexer::consume_until(&Lexer, &formatted[2..], '"', false, linenum);
                            println!(r#"{}"#, stp);
                        }
                    }
                }
                linenum += 1;
            }
        }
    }
}
