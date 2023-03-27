# general syntax

## variables
let int = 34
let str = "hello"
let bool = true
let float = 3.4

## print
"this is a print statement"
f"this is a print statement with vars: {int} {str} {boo} {float}"

## functions
proc helloworld => args {
    "hello world"
}

### func call
helloworld arg1 arg2

## conditionals
if 1 > 2 {
    "1 is bigger than 2"
} elif 1 < 2 {
    "1 is not bigger than 2"
} else {
    "none are true"
}

## for loop
for let i = 2 | i < 10 | i++ {
    f"i is {i}!"
}

## while loop
while 1 < 2 {
    "L"
}

# system sleep
sleep 2