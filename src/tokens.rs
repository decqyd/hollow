pub mod token {

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Token {
        Add,
        Sub,
        Mul,
        Div,
        OpenP,
        CloseP,
        Identifier,
        Loop,
        Conditional,
        Assignment,
    }

    impl Token {
        pub fn new(s: &str) -> Self {
            match s {
                "(" => Self::OpenP,
                ")" => Self::CloseP,
                "make" => Self::Identifier,
                "for" | "while" => Self::Loop,
                "if" | "else if" | "else" => Self::Conditional,
                "=" => Self::Assignment,
                "+" => Self::Add,
                "-" => Self::Sub,
                "*" => Self::Mul,
                "/" => Self::Div,
                _ => panic!("unexpected token"),
            }
        }
    }
}
