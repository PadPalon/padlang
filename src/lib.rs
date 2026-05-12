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

/**
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

pub enum Literal {
    String(String),
    Number(f64),
    True(bool),
    False(bool),
}
impl PrettyPrint for Literal {
    fn pretty_print(&self) -> String {
        match self {
            Literal::String(value) => String::from(value),
            Literal::Number(value) => value.to_string(),
            Literal::True(_) => String::from("true"),
            Literal::False(_) => String::from("false"),
        }
    }
}

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

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Operator,
}
impl PrettyPrint for Binary {
    fn pretty_print(&self) -> String {
        format!("{} {} {}", self.left.pretty_print(), self.operator.pretty_print(), self.right.pretty_print())
    }
}

pub enum Operator {
    And,
    Or,
    Xor,
    Is
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

pub struct Grouping {
    pub expression: Box<Expr>,
}
impl PrettyPrint for Grouping {
    fn pretty_print(&self) -> String {
        format!("({})", self.expression.pretty_print())
    }
}

pub struct Assignment {
    pub identifier: String,
    pub expression: Box<Expr>,
}
impl PrettyPrint for Assignment {
    fn pretty_print(&self) -> String {
        format!("{} = {}", self.identifier, self.expression.pretty_print())
    }
}

