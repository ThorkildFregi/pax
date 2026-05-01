macro_rules! expect_token {
    ($parser:expr, $pattern:pat => $value:expr, $err_msg:expr) => {
        match &$parser.current_token {
            $pattern => { 
                let result = $value;
                $parser.advance();
                result
            }
            _ => return Err(SyntaxError {
                message: $err_msg,
                line: $parser.lexer.line,
            }),
        }
    };

    ($parser:expr, $token:path) => {
        if $parser.current_token == $token {
            $parser.advance();
        } else {
            return Err(SyntaxError {
                message: format!("Syntax Error: Expected {:?}, found {:?}", $token, $parser.current_token),
                line: $parser.lexer.line,
            });
        }
    };
}

macro_rules! parse_print {
    ($parser:expr, $type:ident) => {
       {
            $parser.advance();

            expect_token!($parser, Token::LeftBracket);

            let value = $parser.parse_expression()?;

            expect_token!($parser, Token::RightBracket);
            
            expect_token!($parser, Token::Semicolon);

            Ok(Stmt::$type { value })
        }
    };
}

macro_rules! parse_math_op {
    ($parser:expr, $next_method:ident, $conditions:expr) => {
        {
            let mut left = $parser.$next_method()?;

            while $conditions {
                let operator = $parser.current_token.clone();

                $parser.advance();

                let right = $parser.$next_method()?;

                left = Expr::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            }

            left
        }
    };
}

use crate::token::Token;
use crate::lexer::Lexer;

#[derive(Debug)]
pub enum Expr {
    Integer(i64),

    String(String),

    Boolean(bool),

    Variable(String),

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line: u32,
}

#[derive(Debug)]
pub struct Condition {
    pub condition: Expr,
    pub program: Program,
}

#[derive(Debug)]
pub enum Stmt {
    VarDeclaration {
        name: String,
        value: Expr,
        is_constant: bool,
        is_global: bool,
    },
    VarChange {
        name: String,
        value: Expr,
    },

    ConditionChain {
        conditions: Vec<Condition>,
        else_condition: Option<Program>,
    },

    Print {
        value: Expr,
    },
    Println {
        value: Expr,
    },
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self{
        let first = lexer.next_token();
        Self { 
            lexer, 
            current_token: first
        }
    }

    pub fn parse_program(&mut self, end_token: Token) -> Result<Program, SyntaxError> {
        let mut program = Program { statements: Vec::new() };

        while self.current_token != end_token {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
        }

        Ok(program)
    }

    fn parse_expression(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_equal, self.current_token == Token::And || self.current_token == Token::Or))
    }

    fn parse_equal(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_addition, self.current_token == Token::Equal || self.current_token == Token::Different || self.current_token == Token::More || self.current_token == Token::Less || self.current_token == Token::MoreEqual || self.current_token == Token::LessEqual))
    }

    fn parse_addition(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_multiplication, self.current_token == Token::Plus || self.current_token == Token::Minus))
    }

    fn parse_multiplication(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_power, self.current_token == Token::Star || self.current_token == Token::Slash))
    }

    fn parse_power(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_unary, self.current_token == Token::Caret))
    }

    fn parse_unary(&mut self) -> Result<Expr, SyntaxError> {
        if self.current_token == Token::Not {
            let operator = self.current_token.clone();
            self.advance();

            let right = self.parse_unary()?;

            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, SyntaxError> {
        match &self.current_token {
            Token::Integer(n) => {
                let val = n.clone();
                self.advance();
                Ok(Expr::Integer(val))
            }

            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                Ok(Expr::Variable(var_name))
            }

            Token::String(string) => {
                let string = string.clone();
                self.advance();
                Ok(Expr::String(string))
            }

            Token::Boolean(b) => {
                let value = *b;
                self.advance();
                Ok(Expr::Boolean(value))
            }

            _ => Err(SyntaxError {
                message: format!("Unknown expression: {:?}", self.current_token),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, SyntaxError> {
        match &self.current_token {
            Token::Error(msg) => Err(SyntaxError { 
                message: format!("Lexical error: {}", msg),
                line: self.lexer.line,
            }),

            Token::KeywordVar => self.parse_var(true),
            Token::Identifier(_) => self.parse_var(false),

            Token::KeywordPrint => parse_print!(self, Print),
            Token::KeywordPrintln => parse_print!(self, Println),

            Token::KeywordIf => self.parse_condition(),

            _ => Err(SyntaxError {
                message: format!("Unexpected token: {:?}", self.current_token),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_var(&mut self, is_declaration: bool) -> Result<Stmt, SyntaxError> {
        let mut is_constant = false;
        let mut is_global = false;
        
        if is_declaration {
            self.advance();
            
            while self.current_token == Token::KeywordConst || self.current_token == Token::KeywordGlobal {
                match self.current_token {
                    Token::KeywordConst => {
                        is_constant = true;
                        self.advance();
                    },
                    Token::KeywordGlobal => {
                        is_global = true;
                        self.advance();
                    },
                    _ => break,
                }
            }
        }

        let name = expect_token!(self, Token::Identifier(n) => n.clone(), "Expected variable name after 'var'".to_string());

        expect_token!(self, Token::Assign);

        let value = self.parse_expression()?;

        expect_token!(self, Token::Semicolon);

        if is_declaration {
            Ok(Stmt::VarDeclaration { name, value, is_constant, is_global }) 
        } else {
            Ok(Stmt::VarChange { name, value })
        }
    }

    fn parse_condition(&mut self) -> Result<Stmt, SyntaxError> {
        let mut conditions = Vec::new();
        let mut else_condition = None;

        while self.current_token == Token::KeywordIf || self.current_token == Token::KeywordElif {
            self.advance();

            let condition = self.parse_expression()?;

            expect_token!(self, Token::LeftCurlyBracket);

            let program = self.parse_program(Token::RightCurlyBracket)?;

            self.advance();

            conditions.push(Condition { condition, program });
        }

        if self.current_token == Token::KeywordElse {
            self.advance();

            expect_token!(self, Token::LeftCurlyBracket);

            let else_program = self.parse_program(Token::RightCurlyBracket)?;

            self.advance();

            else_condition = Some(else_program);
        }

        Ok(Stmt::ConditionChain { conditions, else_condition })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}