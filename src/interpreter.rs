use std::collections::HashMap;

use crate::token::Token;
use crate::parser::{Program, Stmt, Expr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),

    String(String),

    Boolean(bool),

    List(Vec<Value>),
}

#[derive(Clone)]
struct VariableSlot {
    value: Value,
    is_constant: bool,
}

pub struct Interpreter {
    scope_stack: Vec<HashMap<String, VariableSlot>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![HashMap::new()],
        }
    }

    pub fn run(&mut self, program: Program) {
        for stmt in program.statements {
            match stmt {
                Stmt::VarDeclaration { name, value, is_constant, is_global } => {
                    match self.evaluate(value) {
                        Ok(res) => {
                            if is_global {
                                self.scope_stack[0].insert(name, VariableSlot { value: res, is_constant });
                            } else {
                                self.scope_stack.last_mut().unwrap().insert(name, VariableSlot { value: res, is_constant });
                            }
                        }
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    }
                }
                Stmt::VarChange { name, value } => {
                    match self.evaluate(value) {
                        Ok(new_val) => {
                            let mut found = false;
                            for scope in self.scope_stack.iter_mut().rev() {
                                if let Some(slot) = scope.get_mut(&name) {
                                    if slot.is_constant {
                                        eprintln!("Runtime Error: Can't modify constant '{}'", name);
                                        return;
                                    }

                                    if std::mem::discriminant(&slot.value) == std::mem::discriminant(&new_val) {
                                        slot.value = new_val;
                                        found = true;
                                        break;
                                    } else {
                                        eprintln!("Runtime Error: TypeError for '{}'", name);
                                        return;
                                    }
                                }
                            }
                            if !found {
                                eprintln!("Runtime Error: Variable '{}' not found", name);
                                return;
                            }
                        },
                        Err(e) => { eprintln!("{}", e); return; }
                    }
                }
                Stmt::ListChange { name, index, value } => {
                    let new_val = match self.evaluate(value) {
                        Ok(v) => v,
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    };

                    let idx = match self.evaluate(index) {
                        Ok(Value::Integer(i)) => i,
                        Ok(_) => { eprintln!("Runtime Error: Index must be an integer"); return; }
                        Err(e) => { eprintln!("Runtime Error: {}", e); return; }
                    };

                    let mut found = false;
                    for scope in self.scope_stack.iter_mut().rev() {
                        if let Some(slot) = scope.get_mut(&name) {
                            if slot.is_constant {
                                eprintln!("Runtime Error: Can't modify constant '{}'", name);
                                return;
                            }

                            match &mut slot.value {
                                Value::List(list) => {
                                    if idx >= 0 && (idx as usize) < list.len() {
                                        list[idx as usize] = new_val;
                                    } else {
                                        eprintln!("Runtime Error: Index {} out of bounds for list '{}'", idx, name);
                                        return;
                                    }
                                }
                                _ => {
                                    eprintln!("Runtime Error: Variable '{}' is not a list", name);
                                    return;
                                }
                            }
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        eprintln!("Runtime Error: Variable '{}' not found", name);
                    }
                }

                Stmt::ConditionChain { conditions, else_condition } => {
                    let mut executed = false;

                    for cond in conditions {
                        match self.evaluate(cond.condition) {
                            Ok(res) => {
                                if let Value::Boolean(b) = res {
                                    if b {
                                        self.scope_stack.push(HashMap::new());
                                        self.run(cond.program);
                                        self.scope_stack.pop();
                                        
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
                            self.scope_stack.push(HashMap::new());
                            self.run(prog);
                            self.scope_stack.pop();
                        }
                    }
                }

                Stmt::While { condition, program } => {
                    loop {
                        match self.evaluate(condition.clone()) {
                            Ok(Value::Boolean(true)) => {
                                self.scope_stack.push(HashMap::new());
                                self.run(program.clone()); 
                                self.scope_stack.pop();
                            }
                            Ok(Value::Boolean(false)) => break,
                            Ok(_) => {
                                eprintln!("Runtime Error: Condition must be a Boolean");
                                return;
                            }
                            Err(e) => {
                                eprintln!("Runtime Error: {}", e);
                                return;
                            }   
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

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Integer(n) => Ok(Value::Integer(n)),
            Expr::Variable(name) => {
                for scope in self.scope_stack.iter().rev() {
                    if let Some(slot) = scope.get(&name) {
                        return Ok(slot.value.clone());
                    }
                }
                Err(format!("Runtime Error: Variable '{}' not found", name))
            }
            Expr::String(string) => Ok(Value::String(string)),
            Expr::Boolean(boolean) => Ok(Value::Boolean(boolean)),
            Expr::List(elements) => {
                let mut elems = Vec::new();
                
                for element in elements {
                    elems.push(self.evaluate(element)?);
                }

                Ok(Value::List(elems))
            }
            Expr::Unary { operator, right } => {
                let res = self.evaluate(*right)?;

                match operator {
                    Token::Not => {
                        if let Value::Boolean(b) = res {
                            Ok(Value::Boolean(!b))
                        } else {
                            Err("TypeError: '!' operator expected a boolean".into())
                        }
                    }
                    _ => Err(format!("Unknown unary operator: {:?}", operator)),
                }
            }
            Expr::Binary {left, operator, right} => {
                let l = self.evaluate(*left)?;
                let r = self.evaluate(*right)?;

                match operator {
                    Token::And => match (l.clone(), r.clone()) {
                        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a & b)),
                        _ => Err(format!("TypeError: logical binary operator requires boolean operands, but found '{:?}' and '{:?}'", l, r))
                    },
                    Token::Or => match (l.clone(), r.clone()) {
                        (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a | b)),
                        _ => Err(format!("TypeError: logical binary operator requires boolean operands, but found '{:?}' and '{:?}'", l, r))
                    },
                    Token::Equal => Ok(Value::Boolean(l == r)),
                    Token::Different => Ok(Value::Boolean(l != r)),
                    Token::More => Ok(Value::Boolean(l > r)),
                    Token::Less => Ok(Value::Boolean(l < r)),
                    Token::MoreEqual => Ok(Value::Boolean(l >= r)),
                    Token::LessEqual => Ok(Value::Boolean(l <= r)),
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
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(elems) => {
                write!(f, "[")?;
                
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if matches!(elem, Value::String(_)) {
                        write!(f, "\"{}\"", elem)?;
                    } else {
                        write!(f, "{}", elem)?;
                    }
                }
                
                write!(f, "]")
            }
        }
    }
}