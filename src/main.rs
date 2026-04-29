mod token;
mod lexer;
mod parser;
mod interpreter;

use std::fs;
use std::env;
use std::process;

use token::Token;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        println!("Pax Language v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.len() < 2 {
        println!("Usage: pax <filename.pax>");
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".pax") {
        println!("Warning: Running a file without .pax extension");
    }

    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    run_compiler(source);
}

fn run_compiler(source: String) {
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer);

    match parser.parse_program(Token::EOF) {
        Ok(program) => {
            let mut interpreter = Interpreter::new();
            interpreter.run(program);
            print!("\n");
        }

        Err(e) => {
            println!("Error at line {}: {}", e.line, e.message);
        }
    }
}