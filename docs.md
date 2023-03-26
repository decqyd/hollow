# docs
* just for custom functions so i can remember what they do

## lexer.rs
### consume_until(
    (
        string: &str,
        until: char,
        inclusive: boolean,
        linenumber: i32,
    ) -> &str)
* creates a new string slice from "string" from the start until it finds "until"
* inclusive decides whether to include until in the final string slice
* linenumber is just used for errors
* returns string slice

### find_char(
    (
    s: &str, find: char
    ) -> Option<i32>)
* similar to consume until but instead returns index of "find" in the string "s"

