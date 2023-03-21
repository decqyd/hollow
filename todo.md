# todo

## syntax
* draw up syntax design

## types
* add more types

## tokens
* add more tokens

## lexer
* add custom fields to each lex:
[{FUNCTIONINDENT: "proc", VALUE: "function name", ARGS: "args"}]
[{IDENTIFIER: "let", VARVALUE: "4", VARNAME: "x", ASSIGNMENT: "="}]

## parser
* make parser parse and eval properly
* set expected formats for all tokens, if lexed code doesnt match example, produce token error!
* finish implementing all keyword commands and lexing formats
! work out how to link vars hashmap between two functions
    * could make a seperate function idk
    * or factor parse and eval function by make it only parse the whole file rather than each line, leave that up to eval func!