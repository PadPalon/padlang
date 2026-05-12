use clap::Parser;
use padlang::lexer::scan_tokens;
use padlang::{Binary, Expr, Grouping, Literal, Operator, PrettyPrint};
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("input.pad"))]
    source_file: String,
}

fn main() {
    // let args = Args::parse();
    // let sourcecode = fs::read_to_string(args.source_file).expect("Cannot read source file");
    // let tokens = match scan_tokens(sourcecode) {
    //     Ok(tokens) => tokens,
    //     Err(error_message) => panic!("{}", error_message),
    // };
    // println!("Tokens:");
    // for token in &tokens {
    //     println!("- {}", token.to_string())
    // }

    let literal_left = Literal::String(String::from("left"));
    let literal_right = Literal::String(String::from("left"));
    let literal_expression_left = Expr::Literal(literal_left);
    let literal_expression_right = Expr::Literal(literal_right);
    let grouping_left = Grouping {
        expression: Box::new(literal_expression_left),
    };
    let grouping_right = Grouping {
        expression: Box::new(literal_expression_right),
    };
    let expr = Expr::Binary(Box::new(Binary {
        left: Box::new(Expr::Grouping(Box::new(grouping_left))),
        operator: Operator::Xor,
        right: Box::new(Expr::Grouping(Box::new(grouping_right))),
    }));
    println!("{}", expr.pretty_print());
}
