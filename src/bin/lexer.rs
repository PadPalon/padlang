use clap::Parser;
use padlang::lexer::scan_tokens;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("input.pad"))]
    source_file: String,
}

fn main() {
    let args = Args::parse();
    let sourcecode = fs::read_to_string(args.source_file).expect("Cannot read source file");
    let tokens = match scan_tokens(sourcecode) {
        Ok(tokens) => tokens,
        Err(error_message) => panic!("{}", error_message),
    };
    println!("Tokens:");
    for token in &tokens {
        println!("- {}", token.to_string())
    }
}
