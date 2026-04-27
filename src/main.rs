mod token;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

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
            let mut interpreter = Interpreter::new();
            interpreter.run(program);
            
            // Pour vérifier que ça marche, on affiche la mémoire à la fin
            println!("Mémoire finale : {:?}", interpreter.variables);
        }

        Err(e) => {
            println!("Error at line {}: {}", e.line, e.message);
        }
    }
}