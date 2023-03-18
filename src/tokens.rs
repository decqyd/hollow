pub mod token {
    #[derive(Debug, PartialEq, Eq, Hash)]

    pub enum Token {
        ADD,
        ADDEQUALS,
        SUB,
        SUBEQUALS,
        MUL,
        MULEQUALS,
        DIV,
        DIVEQUALS,
        OPENP,
        CLOSEP,
        IDENTIFIER,
        FORLOOP,
        FORSEPERATOR,
        WHILELOOP,
        ASSIGNMENT,
        VARNAME,
        VARVALUE,
        SEMICOLON,
        DOT,
        CURLYOP,
        CURLYCP,
        FUNCTIONIDENT,
        FATARROW,
        BANG,
        BANGEQUAL,
        EQUAL,
        GREATER,
        GREATEREQUAL,
        LESS,
        LESSEQUAL,
        IF,
        ELIF,
        ELSE,
        OR,
        AND,
        TRUE,
        FALSE,
        NEWLINE,
        STRING,
    }

    impl Token {
        pub fn new(s: &str) -> Self {
            match s {
                "+" => Self::ADD,
                "+=" => Self::ADDEQUALS,
                "-" => Self::SUB,
                "-=" => Self::SUBEQUALS,
                "*" => Self::MUL,
                "*=" => Self::MULEQUALS,
                "/" => Self::DIV,
                "/=" => Self::DIVEQUALS,
                "(" => Self::OPENP,
                ")" => Self::CLOSEP,
                "let" => Self::IDENTIFIER,
                "for" => Self::FORLOOP,
                "|" => Self::FORSEPERATOR,
                "while" => Self::WHILELOOP,
                "=" => Self::ASSIGNMENT,
                "varname" => Self::VARNAME,
                "varvalue" => Self::VARVALUE,
                ";" => Self::SEMICOLON,
                "." => Self::DOT,
                "\"" => Self::STRING,
                "{" => Self::CURLYOP,
                "}" => Self::CURLYCP,
                "proc" => Self::FUNCTIONIDENT,
                "=>" => Self::FATARROW,
                "!" => Self::BANG,
                "!=" => Self::BANGEQUAL,
                ">" => Self::GREATER,
                ">=" => Self::GREATEREQUAL,
                "<" => Self::LESS,
                "<=" => Self::LESSEQUAL,
                "if" => Self::IF,
                "elif" => Self::ELIF,
                "else" => Self::ELSE,
                "||" => Self::OR,
                "&&" => Self::AND,
                "true" => Self::TRUE,
                "false" => Self::FALSE,
                "\n" => Self::NEWLINE,
                _ => {
                    println!("unexpected token: {s}");
                    Self::ADDEQUALS
                }
            }
        }
    }
}
