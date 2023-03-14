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
