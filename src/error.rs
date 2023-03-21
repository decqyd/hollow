use colour::*;

pub mod error {

    #[derive(Debug)]
    pub struct Error;

    #[derive(Debug)]
    pub enum ErrorType {
        TokenError,
        TypeError,
    }

    impl Error {
        pub fn new(errortype: ErrorType, msg: String, linenum: i32) {
            colour::red!("{:#?}: {msg} on line {}\n", errortype, linenum + 1);
            std::process::exit(69);
        }
    }
}
