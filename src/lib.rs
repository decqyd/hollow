#![allow(unused_imports, dead_code)]

pub mod error;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod types;
use crate::tokens::Token;
use regex::Regex;
use std::collections::HashMap;
use types::{Float, Integer};

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn parse_number() {
        let num = "1234";
        assert_eq!(Integer(1234), Integer::new(num))
    }

    #[test]
    fn parse_float() {
        let f = "1.234";
        assert_eq!(Float(1.234), Float::new(f))
    }

    #[test]
    fn parse_plus_token() {
        assert_eq!(Token::ADD, Token::new("+"))
    }

    #[test]
    fn parse_openp_token() {
        assert_eq!(Token::OPENP, Token::new("("))
    }
}
