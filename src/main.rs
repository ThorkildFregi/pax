mod token;
mod lexer;
mod parser;
mod interpreter;

use std::fs;
use std::env;
use std::process;

use include_dir::{include_dir, Dir};

use token::Token;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

static DOC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/docs/book");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "docs" => {
                open_docs();
                return;
            }
            "--version" | "-v" => {
                println!("Pax Language v{}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
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

fn open_docs() {
    if let Some(home_dir) = dirs::home_dir() {
        let docs_path = home_dir.join("pax_docs");

        fs::create_dir_all(&docs_path).expect("Failed to create docs directory");

        DOC_DIR.extract(&docs_path).expect("Failed to extract embedded documentation");

        let index_path = docs_path.join("index.html");
        open::that(index_path).expect("Failed to open browser");
    } else {
        eprintln!("Could not find your home directory!");
    }
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