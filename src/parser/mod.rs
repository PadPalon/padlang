use crate::{Expr, Literal, PrettyPrint};
use super::{Token};

pub fn parse(tokens: Vec<Token>) {
    let literal = Literal::String(String::from("test"));
    let expr = Expr::Literal(literal);
    println!("{}", expr.pretty_print());
    return
}