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

macro_rules! parse_collection {
    ($parser:expr, $close_token:expr, $err_msg:expr, $is_map:expr) => {
        {
            $parser.advance();

            if $parser.current_token == $close_token {
                $parser.advance();
                return if $is_map {
                    Ok(Expr::Map(Vec::new()))
                } else {
                    Ok(Expr::List(Vec::new()))
                };
            }

            let mut list_elements = Vec::new();
            let mut map_elements = Vec::new();

            loop {
                if $is_map {
                    let key = $parser.parse_expression()?;
                    expect_token!($parser, Token::Colon);
                    let value = $parser.parse_expression()?;
                    map_elements.push((key, value));
                } else {
                    list_elements.push($parser.parse_expression()?);
                }

                if $parser.current_token == Token::Comma {
                    $parser.advance();
                } else if $parser.current_token == $close_token {
                    break;
                } else {
                    return Err(SyntaxError {
                        message: $err_msg.into(),
                        line: $parser.lexer.line,
                    });
                }
            }

            $parser.advance();

            return if $is_map {
                Ok(Expr::Map(map_elements))
            } else {
                Ok(Expr::List(list_elements))
            };
        }
    };
}

use crate::token::Token;
use crate::lexer::Lexer;

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),

    String(String),

    Boolean(bool),

    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),

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

#[derive(Debug, Clone)]
pub struct Condition {
    pub condition: Expr,
    pub program: Program,
}

#[derive(Debug, Clone)]
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
    CollectionChange {
        name: String,
        index: Expr,
        value: Expr,
    },
    CollectionModification {
        name: String,
        operation: String,
        key: Option<Expr>,
        value: Option<Expr>,
    },

    For {
        name_var: String,
        list: Expr,
        program: Program,
    },
    While {
        condition: Expr,
        program: Program,
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

#[derive(Debug, Clone)]
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

            Token::LeftSquareBracket => parse_collection!(self, Token::RightSquareBracket, "Expected ',' or ']' after map element", false),

            Token::LeftCurlyBracket => parse_collection!(self, Token::RightCurlyBracket, "Expected ',' or '}' after map element", true),

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

            Token::KeywordIf => self.parse_condition(),
            
            Token::KeywordFor => self.parse_for(),
            Token::KeywordWhile => self.parse_while(),

            Token::KeywordPrint => parse_print!(self, Print),
            Token::KeywordPrintln => parse_print!(self, Println),

            _ => Err(SyntaxError {
                message: format!("Unexpected token: {:?}", self.current_token),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_var(&mut self, is_declaration: bool) -> Result<Stmt, SyntaxError> {
        let mut is_constant = false;
        let mut is_global = false;
        let mut index_expr: Option<Expr> = None;
        
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
    
        if self.current_token == Token::LeftSquareBracket {
            self.advance();
            index_expr = Some(self.parse_expression()?);
            expect_token!(self, Token::RightSquareBracket);

        } else if self.current_token == Token::Dot {
            self.advance();

            match self.current_token {
                Token::KeywordAppend => {
                    self.advance();

                    expect_token!(self, Token::LeftBracket);
                    let value = Some(self.parse_expression()?);
                    expect_token!(self, Token::RightBracket);

                    expect_token!(self, Token::Semicolon);

                    return Ok(Stmt::CollectionModification { name, operation: "append".into(), key: None, value: value });
                }
                Token::KeywordPop => {
                    let mut key = None;

                    self.advance();

                    expect_token!(self, Token::LeftBracket);
                    if let Ok(k) = self.parse_expression() {    
                        key = Some(k);
                    }
                    expect_token!(self, Token::RightBracket);

                    expect_token!(self, Token::Semicolon);

                    return Ok(Stmt::CollectionModification { name, operation: "pop".into(), key: key, value: None });
                }
                Token::KeywordRemove => {
                    self.advance();

                    expect_token!(self, Token::LeftBracket);
                    let key = Some(self.parse_expression()?);
                    expect_token!(self, Token::RightBracket);

                    expect_token!(self, Token::Semicolon);

                    return Ok(Stmt::CollectionModification { name, operation: "remove".into(), key: key, value: None });
                }
                _ => return Err(SyntaxError {
                    message: "Keyword unknown".into(),
                    line: self.lexer.line,
                }),
            }
        }

        expect_token!(self, Token::Assign);

        let value = self.parse_expression()?;

        expect_token!(self, Token::Semicolon);

        if is_declaration && index_expr.is_some() {
            return Err(SyntaxError {
                message: "Cannot index a variable during declaration".into(),
                line: self.lexer.line,
            });
        }

        if is_declaration {
            Ok(Stmt::VarDeclaration { name, value, is_constant, is_global }) 
        } else if let Some(index) = index_expr {
            Ok(Stmt::CollectionChange { name, index, value })
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

    fn parse_for(&mut self) -> Result<Stmt, SyntaxError> {
        self.advance();

        let name_var = expect_token!(self, Token::Identifier(n) => n.clone(), "Expected variable name after 'var'".to_string());

        expect_token!(self, Token::KeywordIn);

        let list = self.parse_expression()?;

        expect_token!(self, Token::LeftCurlyBracket);

        let program = self.parse_program(Token::RightCurlyBracket)?;

        self.advance();

        Ok(Stmt::For { name_var, list, program })
    }

    fn parse_while(&mut self) -> Result<Stmt, SyntaxError> {
        self.advance();

        let condition = self.parse_expression()?;

        expect_token!(self, Token::LeftCurlyBracket);

        let program = self.parse_program(Token::RightCurlyBracket)?;

        self.advance();

        Ok(Stmt::While { condition, program })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}