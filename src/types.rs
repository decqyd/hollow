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
pub struct Str(pub String);
impl Str {
    pub fn new(s: &str) -> Self {
        Self(String::from(s))
    }
}

#[derive(Debug, PartialEq)]
pub struct Bool(pub bool);
impl Bool {
    pub fn new(s: &str) -> Self {
        match s {
            "true" | "True" | "TRUE" => Self(true),
            "false" | "False" | "FALSE" => Self(false),
            _ => panic!("not a boolean type"),
        }
    }
}
