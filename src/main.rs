mod token;
mod lexer;

use token::Token;
use lexer::Lexer;

fn main() {
    let code = "var var_name = 10;".to_string();
    println!("Analysis of {}", code);

    let mut lexer = Lexer::new(code);

    loop {
        let t = lexer.next_token();
        println!("Token found: {:?}", t);

        if t == Token::EOF {
            break;
        }

        if let Token::Error(msg) = t {
            println!("Error: {}", msg);
            break;
        }
    }
}