#![allow(unused_imports, unused_variables)]
use hollow::error::{Error, ErrorType};
use hollow::lexer::Lexer;
use hollow::parser::Parser;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!(
            "
    USAGE:
        hollow <filepath>"
        )
    } else {
        let file: PathBuf = PathBuf::from(&args[1]);
        let extension = match file.extension() {
            Some(e) => e,
            None => {
                Error::new(
                    ErrorType::RuntimeError,
                    "File must have a .hlw extension!".to_string(),
                    0,
                );
                unreachable!("L")
            }
        };

        if extension != "hlw" {
            Error::new(
                ErrorType::RuntimeError,
                "File must have a .hlw extension!".to_string(),
                0,
            );
        } else {
            let mut file = File::open(file).expect("couldnt open file");
            let mut data = String::new();
            file.read_to_string(&mut data)
                .expect("couldnt read contents!");
            let _data: &str = &data;
            let p = Parser::new();
            p.eval(&data);
        }
    }
}
