use std::collections::HashMap;

use crate::token::Token;
use crate::parser::{Program, Stmt, Expr};

pub struct Interpreter {
    pub variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Program) {
        for stmt in program.statements {
            match stmt {
                Stmt::VarDeclaration { name, value } => {
                    let result = self.evaluate(value);
                    self.variables.insert(name, result);
                }

                Stmt::Print { value } => {
                    let result = self.evaluate(value);
                    print!("{}", result);
                }

                Stmt::Println { value } => {
                    let result = self.evaluate(value);
                    println!("{}", result);
                }
            }
        }
    }

    fn evaluate(&self, expr: Expr) -> i64 {
        match expr {
            Expr::Integer(n) => n,
            Expr::Variable(name) => {
                *self.variables.get(&name).expect("Variable not found")
            }
            Expr::Binary {left, operator, right} => {
                let l = self.evaluate(*left);
                let r = self.evaluate(*right);

                match operator {
                    Token::Plus => l + r,
                    Token::Minus => l - r,
                    _ => panic!("Operator not supported"),
                }
            }
        }
    }
}