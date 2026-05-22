use super::{Assignment, Binary, Operator, Token, TokenType};
use crate::{Expr, Literal, Program};
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut expressions: Vec<Expr> = Vec::new();
    let mut peekable = tokens.iter().peekable();
    while let Some(&token) = peekable.peek() {
        expressions.push(parse_expression(token, &mut peekable));
    }
    Program { expressions }
}

fn parse_expression(token: &Token, mut tokens: &mut Peekable<Iter<Token>>) -> Expr {
    match token.token_type {
        TokenType::Identifier => {
            let lexeme = token.lexeme.clone().unwrap();
            advance(&mut tokens);
            match tokens.peek() {
                None => parse_identifier_literal(lexeme),
                Some(next_token) => match next_token.token_type {
                    TokenType::Equals => {
                        advance(&mut tokens);
                        parse_assignment(lexeme, &mut tokens)
                    }
                    TokenType::And => {
                        advance(&mut tokens);
                        parse_binary(parse_identifier_literal(lexeme), Operator::And, &mut tokens)
                    }
                    TokenType::Or => {
                        advance(&mut tokens);
                        parse_binary(parse_identifier_literal(lexeme), Operator::Or, &mut tokens)
                    }
                    TokenType::Xor => {
                        advance(&mut tokens);
                        parse_binary(parse_identifier_literal(lexeme), Operator::Xor, &mut tokens)
                    }
                    TokenType::Is => {
                        advance(&mut tokens);
                        parse_binary(parse_identifier_literal(lexeme), Operator::Is, &mut tokens)
                    }
                    _ => {
                        parse_identifier_literal(lexeme)
                    }
                },
            }
        }
        TokenType::String => {
            let lexeme = token.lexeme.clone().unwrap();
            advance(&mut tokens);
            Expr::Literal(Literal::String(lexeme))
        }
        TokenType::Number => {
            let lexeme = token.lexeme.clone().unwrap().parse().unwrap();
            advance(&mut tokens);
            Expr::Literal(Literal::Number(lexeme))
        }
        TokenType::True => {
            advance(&mut tokens);
            Expr::Literal(Literal::True())
        }
        TokenType::False => {
            advance(&mut tokens);
            Expr::Literal(Literal::False())
        }
        _ => {
            println!("Not yet implemented start token {}", token.to_string());
            todo!()
        }
    }
}

fn parse_assignment(identifier: String, mut tokens: &mut &mut Peekable<Iter<Token>>) -> Expr {
    let expression = match tokens.peek() {
        None => {
            panic!("Unexpected end of tokens");
        }
        Some(token) => parse_expression(token, &mut tokens),
    };
    Expr::Assignment(Box::new(Assignment {
        identifier,
        expression: Box::new(expression),
    }))
}

fn parse_binary(
    left: Expr,
    operator: Operator,
    mut tokens: &mut &mut Peekable<Iter<Token>>,
) -> Expr {
    let right = match tokens.peek() {
        None => {
            panic!("Unexpected end of tokens");
        }
        Some(token) => parse_expression(token, &mut tokens),
    };
    Expr::Binary(Box::new(Binary {
        left: Box::new(left),
        right: Box::new(right),
        operator,
    }))
}

fn parse_identifier_literal(identifier: String) -> Expr {
    Expr::Literal(Literal::Identifier(identifier))
}

fn advance(tokens: &mut Peekable<Iter<Token>>) {
    tokens.next();
}
