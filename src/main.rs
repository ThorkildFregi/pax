mod token;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = "
        var alpha = 10;
        var beta = alpha;
    ";

    println!("--- Source Code ---");
    println!("{}", source);
    println!("-------------------");

    let lexer = Lexer::new(source.to_string());

    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(program) => {
            println!("Parsing achieved ! Here is the AST :");

            for (i, stmt) in program.statements.iter().enumerate() {
                println!("Instruction {}: {:?}", i + 1, stmt);
            }
        }

        Err(e) => {
            println!("Error at line {}: {}", e.line, e.message);
        }
    }
}