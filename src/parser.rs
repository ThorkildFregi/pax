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

        let name = match &self.current_token {
            Token::Identifier(n) => n.clone(),
            _ => return Err(SyntaxError {
                message: "Expected variable name after 'var'".to_string(),
                line: self.lexer.line,
            }),
        };

        self.advance();

        self.expect(Token::Assign)?;

        let value = self.parse_expression()?;

        self.expect(Token::Semicolon)?;

        Ok(Stmt::VarDeclaration { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, SyntaxError> {
        self.advance();

        self.expect(Token::LeftBracket)?;

        let value = self.parse_expression()?;

        self.expect(Token::RightBracket)?;
        
        self.expect(Token::Semicolon)?;

        Ok(Stmt::Print { value })
    }

    fn parse_expression(&mut self) -> Result<Expr, SyntaxError> {
        match &self.current_token {
            Token::Integer(n) => {
                let val = *n;
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

    fn expect(&mut self, expected: Token) -> Result<(), SyntaxError> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            return Err(SyntaxError {
                message: format!("Syntax Error: Expected {:?}, found {:?}", expected, self.current_token),
                line: self.lexer.line,
            });
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}