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

    Variable(String),

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
pub enum Stmt {
    VarDeclaration {
        name: String,
        value: Expr,
    },
    Print {
        value: Expr,
    },
    Println {
        value: Expr,
    },
}

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

    pub fn parse_program(&mut self) -> Result<Program, SyntaxError> {
        let mut program = Program { statements: Vec::new() };

        while self.current_token != Token::EOF {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
        }

        Ok(program)
    }

    fn parse_expression(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_multiplication, self.current_token == Token::Plus || self.current_token == Token::Minus))
    }

    fn parse_multiplication(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_power, self.current_token == Token::Star || self.current_token == Token::Slash))
    }

    fn parse_power(&mut self) -> Result<Expr, SyntaxError> {
        Ok(parse_math_op!(self, parse_atom, self.current_token == Token::Caret))
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

            Token::KeywordVar => self.parse_var_declaration(),
            Token::KeywordPrint => parse_print!(self, Print),
            Token::KeywordPrintln => parse_print!(self, Println),

            _ => Err(SyntaxError {
                message: format!("Unexpected token: {:?}", self.current_token),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Stmt, SyntaxError> {
        self.advance();

        let name = expect_token!(self, Token::Identifier(n) => n.clone(), "Expected variable name after 'var'".to_string());

        expect_token!(self, Token::Assign);

        let value = self.parse_expression()?;

        expect_token!(self, Token::Semicolon);

        Ok(Stmt::VarDeclaration { name, value })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}