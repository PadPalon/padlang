# padlang

Writing hobby languages in the age of fucking LLMs by hand to keep the joy alive.
Yes it's incomplete, but it's mine.

Commands in `bin` run the different parts (lexer, parser, vm), using `input.pad` by default as input.

## Tokens

```
LeftScope,
RightScope,
Equals,
Identifier,
String,
Number,
If,
Else,
And,
Or,
Xor,
Not,
Is,
True,
False,
Function,
Return,
```

## Grammar

```
program -> [expression]
expression -> literal | unary | binary | grouping | assignment
literal -> "String" | "Number" | "True" | "False"
unary -> "Not" expression
binary -> expression operator expression
operator -> ("And" | "Or" | "Xor" | "Is")
grouping -> "LeftScope" expression "RightScope"
assignment -> "Identifier" "Equals" expression
```