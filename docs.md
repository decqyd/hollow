# docs
* just for custom functions so i can remember what they do

## lexer.rs
### consume_until
    pub fn consume_until<'a>(
            &self,
            s: &'a str,
            until: char,
            inclusive: bool,
            linenumber: i32,
        ) -> &'a str 
* creates a new string slice from "string" from the start until it finds "until"
* inclusive decides whether to include until in the final string slice
* linenumber is just used for errors
* returns string slice

### find_char
    pub fn find_char(
        &self, s: &str, find: char
        ) -> Option<i32> 
* similar to consume until but instead returns index of "find" in the string "s"

