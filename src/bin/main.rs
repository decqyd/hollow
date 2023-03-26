#![allow(unused_imports, unused_variables)]
use hollow::error::Error;
use hollow::lexer::Lexer;
use hollow::parser::Parser;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!(
            "   USAGE: 
            hollow <filepath>"
        )
    } else {
        let file: PathBuf = PathBuf::from(&args[1]);
        let mut file = File::open(file).expect("couldnt open file");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("couldnt read contents!");
        let _data: &str = &data;
        let p = Parser::new();
        p.eval(&data);
    }
}
