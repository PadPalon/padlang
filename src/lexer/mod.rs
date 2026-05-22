use super::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub fn scan_tokens(sourcecode: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let mut characters = sourcecode.chars().peekable();

    let mut character_number = 1;
    let mut line_number = 1;

    let mut add_error = |message: String, line_number: i32, character_number: i32| {
        errors.push(format!(
            "{} on line {}:{}",
            message, line_number, character_number
        ))
    };

    while let Some(&character) = characters.peek() {
        match character {
            '(' | '{' | '<' | '[' => {
                tokens.push(tokenize_single_character(TokenType::LeftScope));
                advance(&mut characters, &mut character_number);
            }
            ')' | '}' | '>' | ']' => {
                tokens.push(tokenize_single_character(TokenType::RightScope));
                advance(&mut characters, &mut character_number);
            }
            '=' => {
                tokens.push(tokenize_single_character(TokenType::Equals));
                advance(&mut characters, &mut character_number);
            }

            '"' | '\'' => tokens.push(tokenize_string(
                &mut characters,
                &mut character_number,
                character,
            )),

            _ if character.is_digit(10) => {
                match tokenize_number(&mut characters, &mut character_number) {
                    Ok(token) => tokens.push(token),
                    Err(message) => add_error(message, line_number, character_number),
                }
            }

            _ if character.is_alphabetic() => {
                tokens.push(tokenize_code(&mut characters, &mut character_number))
            }

            '\n' => {
                advance(&mut characters, &mut character_number);
                line_number += 1;
                character_number = 0;
            }

            _ if character.is_whitespace() => {
                advance(&mut characters, &mut character_number);
            }

            _ => {
                add_error(
                    format!("Unexpected character '{}'", character),
                    line_number,
                    character_number,
                );
                advance(&mut characters, &mut character_number);
            }
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors.join("\n"))
    }
}

fn tokenize_single_character(token_type: TokenType) -> Token {
    Token {
        token_type,
        lexeme: None,
    }
}

fn tokenize_string(
    mut characters: &mut Peekable<Chars>,
    mut character_number: &mut i32,
    character: char,
) -> Token {
    advance(&mut characters, &mut character_number);

    let mut string_content = String::new();
    while let Some(&string_character) = characters.peek()
        && string_character != character
    {
        string_content.push(string_character);
        advance(&mut characters, &mut character_number);
    }
    advance(&mut characters, &mut character_number);
    Token {
        token_type: TokenType::String,
        lexeme: Some(string_content),
    }
}

fn tokenize_number(
    mut characters: &mut Peekable<Chars>,
    mut character_number: &mut i32,
) -> Result<Token, String> {
    let mut dot_found = false;
    let mut number_content = String::new();
    while let Some(&number_character) = characters.peek()
        && (number_character.is_digit(10) || number_character == '_' || number_character == '.')
    {
        if number_character.is_digit(10) || number_character == '.' {
            if number_character == '.' {
                if dot_found {
                    return Err(String::from("Multiple dots found in number"));
                } else {
                    dot_found = true;
                }
            }
            number_content.push(number_character);
        }
        advance(&mut characters, &mut character_number);
    }

    if number_content.ends_with(".") {
        number_content.truncate(number_content.len() - 1);
    }

    Ok(Token {
        token_type: TokenType::Number,
        lexeme: Some(number_content),
    })
}

fn tokenize_code(mut characters: &mut Peekable<Chars>, mut character_number: &mut i32) -> Token {
    let mut identifier_content = String::new();
    while let Some(&identifier_character) = characters.peek()
        && (identifier_character.is_alphabetic() || identifier_character.is_digit(10))
    {
        identifier_content.push(identifier_character);
        advance(&mut characters, &mut character_number);
    }

    match_keyword(identifier_content.as_str())
        .map(|token_type| Token {
            token_type,
            lexeme: None,
        })
        .unwrap_or(Token {
            token_type: TokenType::Identifier,
            lexeme: Some(identifier_content),
        })
}

fn match_keyword(keyword: &str) -> Option<TokenType> {
    match keyword {
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "and" => Some(TokenType::And),
        "or" => Some(TokenType::Or),
        "xor" => Some(TokenType::Xor),
        "not" => Some(TokenType::Not),
        "is" => Some(TokenType::Is),
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        "function" => Some(TokenType::Function),
        "return" => Some(TokenType::Return),
        _ => None,
    }
}

fn advance(characters: &mut Peekable<Chars>, character_number: &mut i32) {
    characters.next();
    *character_number += 1;
}
