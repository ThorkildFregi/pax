use std::collections::HashMap;

use crate::token::Token;
use crate::parser::{Program, Stmt, Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),

    String(String),

    Boolean(bool),
}

#[derive(Clone)]
struct VariableSlot {
    value: Value,
    is_constant: bool,
}

pub struct Interpreter {
    variables: HashMap<String, VariableSlot>,
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
                Stmt::VarDeclaration { name, value, is_constant } => {
                    match self.evaluate(value) {
                        Ok(res) => { self.variables.insert(name, VariableSlot { value: res, is_constant }); },
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    }
                }
                Stmt::VarChange { name, value } => {
                    match self.evaluate(value) {
                        Ok(new_val) => {
                            if let Some(slot) = self.variables.get_mut(&name) {
                                if slot.is_constant {
                                    eprintln!("Runtime Error: Can't modify constant variable '{}'", name);
                                    return;
                                }

                                if std::mem::discriminant(&slot.value) == std::mem::discriminant(&new_val) {
                                    slot.value = new_val;
                                } else {
                                    eprintln!("Runtime Error: TypeError: Variable '{}' must remain of the same type", name);
                                    return;
                                }
                            } else {
                                eprintln!("Runtime Error: Variable '{}' not declared. Use 'var' first.", name);
                                return;
                            }
                        },
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    }
                }

                Stmt::ConditionChain { conditions, else_condition } => {
                    let mut executed = false;

                    for cond in conditions {
                        match self.evaluate(cond.condition) {
                            Ok(res) => {
                                if let Value::Boolean(b) = res {
                                    if b {
                                        self.run(cond.program);
                                        executed = true; 
                                        break;
                                    }
                                } else {
                                    eprintln!("Runtime Error: Condition must be a Boolean");
                                    return;
                                }
                            }
                            Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                        }
                    }
                    if !executed {
                        if let Some(prog) = else_condition {
                            self.run(prog);
                        }
                    }
                }

                Stmt::Print { value } => {
                    match self.evaluate(value) {
                        Ok(res) => print!("{}", res),
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    }
                }
                Stmt::Println { value } => {
                    match self.evaluate(value) {
                        Ok(res) => println!("{}", res),
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    }
                }
            }
        }
    }

    fn evaluate(&self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Integer(n) => Ok(Value::Integer(n)),
            Expr::Variable(name) => {
                Ok(self.variables.get(&name).cloned().ok_or_else(|| format!("Variable '{}' not found", name)).unwrap().value)
            }
            Expr::String(string) => Ok(Value::String(string)),
            Expr::Boolean(boolean) => Ok(Value::Boolean(boolean)),
            Expr::Binary {left, operator, right} => {
                let l = self.evaluate(*left)?;
                let r = self.evaluate(*right)?;

                match operator {
                    Token::Equal => Ok(Value::Boolean(l == r)),
                    Token::Different => Ok(Value::Boolean(l != r)),
                    Token::Plus => match (l.clone(), r.clone()) {
                        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                        _ => Err(format!("TypeError: cannot add '{:?}' and '{:?}'", l, r)),
                    },
                    Token::Minus => match (l.clone(), r.clone()) {
                        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                        _ => Err(format!("TypeError: cannot substract '{:?}' and '{:?}'", l, r)),
                    },
                    Token::Star => match (l.clone(), r.clone()) {
                        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                        _ => Err(format!("TypeError: cannot multiply '{:?}' and '{:?}'", l, r))
                    },
                    Token::Slash => match (l.clone(), r.clone()) {
                        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
                        _ => Err(format!("TypeError: cannot divide '{:?}' and '{:?}'", l, r))
                    },
                    Token::Caret => match (l.clone(), r.clone()) {
                        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a.pow(b as u32))),
                        _ => Err(format!("TypeError: cannot power '{:?}' and '{:?}'", l, r))
                    },
                    _ => Err("Operator not supported".into()),
                }
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b)
        }
    }
}