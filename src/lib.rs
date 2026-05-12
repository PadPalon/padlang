use std::fmt;

pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub enum TokenType {
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
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(lexeme) = &self.lexeme {
            write!(f, "{} {}", self.token_type.to_string(), lexeme)
        } else {
            write!(f, "{}", self.token_type.to_string())
        }
    }
}
