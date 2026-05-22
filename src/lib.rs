use std::collections::HashMap;
use std::fmt;

pub mod lexer;
pub mod parser;
pub mod vm;

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

/**
program -> [expression]
expression -> literal | unary | binary | grouping | assignment
literal -> "String" | "Number" | "True" | "False"
unary -> "Not" expression
binary -> expression operator expression
operator -> ("And" | "Or" | "Xor" | "Is")
grouping -> "LeftScope" expression "RightScope"
assignment -> "Identifier" "Equals" expression
**/
pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

pub struct Program {
    pub expressions: Vec<Expr>,
}
impl PrettyPrint for Program {
    fn pretty_print(&self) -> String {
        self.expressions.iter().fold(String::new(), |result, expr| {
            result + "\n" + &expr.pretty_print()
        })
    }
}

#[derive(Clone)]
pub enum Expr {
    Literal(Literal),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Assignment(Box<Assignment>),
}
impl PrettyPrint for Expr {
    fn pretty_print(&self) -> String {
        match self {
            Expr::Literal(expr) => expr.pretty_print(),
            Expr::Unary(expr) => expr.pretty_print(),
            Expr::Binary(expr) => expr.pretty_print(),
            Expr::Grouping(expr) => expr.pretty_print(),
            Expr::Assignment(expr) => expr.pretty_print(),
        }
    }
}

#[derive(Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    True(),
    False(),
    Identifier(String),
}
impl PrettyPrint for Literal {
    fn pretty_print(&self) -> String {
        match self {
            Literal::String(value) => String::from(value),
            Literal::Number(value) => value.to_string(),
            Literal::True() => String::from("true"),
            Literal::False() => String::from("false"),
            Literal::Identifier(value) => String::from(value),
        }
    }
}

#[derive(Clone)]
pub enum Unary {
    Not(Box<Expr>),
}
impl PrettyPrint for Unary {
    fn pretty_print(&self) -> String {
        match self {
            Unary::Not(expr) => format!("not {}", expr.pretty_print()),
        }
    }
}

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Operator,
}
impl PrettyPrint for Binary {
    fn pretty_print(&self) -> String {
        format!(
            "{} {} {}",
            self.left.pretty_print(),
            self.operator.pretty_print(),
            self.right.pretty_print()
        )
    }
}

#[derive(Clone)]
pub enum Operator {
    And,
    Or,
    Xor,
    Is,
}
impl PrettyPrint for Operator {
    fn pretty_print(&self) -> String {
        match self {
            Operator::And => String::from("and"),
            Operator::Or => String::from("or"),
            Operator::Xor => String::from("xor"),
            Operator::Is => String::from("is"),
        }
    }
}

#[derive(Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}
impl PrettyPrint for Grouping {
    fn pretty_print(&self) -> String {
        format!("({})", self.expression.pretty_print())
    }
}

#[derive(Clone)]
pub struct Assignment {
    pub identifier: String,
    pub expression: Box<Expr>,
}
impl PrettyPrint for Assignment {
    fn pretty_print(&self) -> String {
        format!("{} = {}", self.identifier, self.expression.pretty_print())
    }
}

#[derive(PartialEq)]
pub enum StateValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl fmt::Display for StateValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = match self {
            StateValue::String(value) => {
                format!("{}", value)
            }
            StateValue::Number(value) => {
                format!("{}", value)
            }
            StateValue::Boolean(value) => {
                format!("{}", value)
            }
        };
        write!(f, "{}", formatted)
    }
}

pub struct State {
    pub strings: HashMap<String, String>,
    pub numbers: HashMap<String, f64>,
    pub booleans: HashMap<String, bool>,
}
