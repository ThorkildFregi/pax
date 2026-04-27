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

macro_rules! parse_atom {
    ($parser:expr) => {
        match &$parser.current_token {
            Token::Integer(n) => {
                let val = n.clone();
                $parser.advance();
                Expr::Integer(val)
            }

            Token::Identifier(name) => {
                let var_name = name.clone();
                $parser.advance();
                Expr::Variable(var_name)
            }

            _ => return Err(SyntaxError {
                message: format!("Unknown expression: {:?}", $parser.current_token),
                line: $parser.lexer.line,
            }),
        }
    };
}

use crate::token::Token;
use crate::lexer::Lexer;

#[derive(Debug)]
pub enum Expr {
    Integer(i64),

    Variable(String),
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

    fn parse_statement(&mut self) -> Result<Stmt, SyntaxError> {
        match &self.current_token {
            Token::Error(msg) => Err(SyntaxError { 
                message: format!("Lexical error: {}", msg),
                line: self.lexer.line,
            }),

            Token::KeywordVar => self.parse_var_declaration(),
            Token::KeywordPrint => self.parse_print(),

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

        let value = parse_atom!(self);

        expect_token!(self, Token::Semicolon);

        Ok(Stmt::VarDeclaration { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, SyntaxError> {
        self.advance();

        expect_token!(self, Token::LeftBracket);

        let value = parse_atom!(self);

        expect_token!(self, Token::RightBracket);
        
        expect_token!(self, Token::Semicolon);

        Ok(Stmt::Print { value })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}