#![allow(unused_imports, dead_code)]
mod lexer;
mod types;

use types::{Float, Integer, Op};

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
    fn parse_add() {
        assert_eq!(Op::new("+"), Op::Add)
    }

    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), Op::Sub)
    }
}
