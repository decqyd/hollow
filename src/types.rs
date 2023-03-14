#![allow(dead_code, unused_imports)]

#[derive(Debug, PartialEq)]
pub struct Integer(pub i32);
impl Integer {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub struct Float(pub f32);
impl Float {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("bad operator i think"),
        }
    }
}
