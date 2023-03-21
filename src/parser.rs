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
            // make this the main vars hashmap somehow
            let mut vars: HashMap<&str, &str> = HashMap::new();
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
            for h in s {
                for (i, k) in h.iter() {
                    if i == &Token::STRING {
                        println!(r"{}", k);
                    } else if i == &Token::IDENTIFIER {
                        todo!("somehow link vars to main function so vars doesnt get reset");
                        //vars.insert(h[&Token::VARNAME], h[&Token::VARVALUE]);
                    } /* else if i == &Token::FSTRING {
                          let fstring = h[&Token::FSTRING];
                          let stc = h[&Token::STRING];
                          let fvalue = h[&Token::VARVALUE];
                          let varva
                          let formatted = fstring.replace(&stc, &vars[fvalue]);
                          println!(r"{}", formatted);
                      } */
                }
                //println!("vars is {:?}", vars);
            }
        }
    }
}
