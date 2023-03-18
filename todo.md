# todo

## syntax
* draw up syntax design

## types
* add more types

## tokens
* make a tokens list for every token (wip)

## lexer
* make a consume until function
* consume characters from parameter til
a certain character then return and tokenise that.
* consume until allows for individual char checking
* add "value" and "args" field to each lex:
[{FUNCTIONINDENT: "proc", VALUE: "function name", ARGS: "args"}]

## parser
* make parser parse and eval properly
* set expected formats for all tokens, if lexed code doesnt match example, produce token error
