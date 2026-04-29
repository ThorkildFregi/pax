use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    state: u32,
    pub line: u32
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            state: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        self.state = 0;

        let mut value = String::new();

        while self.pos < self.input.len() {
            let ch = self.input[self.pos];

            match self.state {
                0 => match ch {
                    'a'..='z' | 'A'..='Z' => {
                        self.state = 1;
                        value.push(ch);
                        self.pos += 1;
                    }
                    '0'..='9' => {
                        self.state = 2;
                        value.push(ch);
                        self.pos += 1;
                    }
                    '=' => {
                        self.state = 4;
                        self.pos += 1;
                    }
                    '!' => {
                        self.state = 5;
                        self.pos += 1;
                    }
                    '(' => {
                        self.pos += 1;
                        return Token::LeftBracket;
                    }
                    ')' => {
                        self.pos += 1;
                        return Token::RightBracket;
                    }
                    '{' => {
                        self.pos += 1;
                        return Token::LeftCurlyBracket;
                    }
                    '}' => {
                        self.pos += 1;
                        return Token::RightCurlyBracket;
                    }
                    '&' => {
                        self.state = 6;
                        self.pos += 1;
                    }
                    '|' => {
                        self.state = 7;
                        self.pos += 1;
                    }
                    '"' => {
                        self.state = 3;
                        self.pos += 1;
                    }
                    '+' => {
                        self.pos += 1;
                        return Token::Plus;
                    }
                    '-' => {
                        self.pos += 1;
                        return Token::Minus;
                    }
                    '*' => {
                        self.pos += 1;
                        return Token::Star;
                    }
                    '/' => {
                        self.pos += 1;
                        return Token::Slash;
                    }
                    '^' => {
                        self.pos += 1;
                        return Token::Caret;
                    }
                    ';' => {
                        self.pos += 1;
                        return Token::Semicolon;
                    }
                    ' ' | '\t' | '\r' => {
                        self.pos += 1;
                        continue;
                    }
                    '\n' => {
                        self.pos += 1;
                        self.line += 1;
                        continue;
                    }
                    _ => return Token::Error(format!("Character '{}' not supported at line {}", ch, self.line)),
                }
                1 => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                        value.push(ch);
                        self.pos += 1;
                    }
                    _ => break,
                }
                2 => match ch {
                    '0'..='9' => {
                        value.push(ch);
                        self.pos += 1;
                    }
                    _ => break,
                }
                3 => match ch {
                    '"' => {
                        self.pos += 1;
                        break;
                    }
                    _ => {
                        value.push(ch);
                        self.pos += 1;
                    }
                }
                4 => match ch {
                    '=' => {
                        self.pos += 1;
                        return Token::Equal;
                    }
                    _ => {
                        return Token::Assign;
                    }
                }
                5 => match ch {
                    '=' => {
                        self.pos += 1;
                        return Token::Different;
                    }
                    _ => return Token::Error(format!("Character '{}' not supported at line {}", ch, self.line)),
                }
                6 => match ch {
                    '&' => {
                        self.pos += 1;
                        return Token::Or;
                    }
                    _ => return Token::Error(format!("Character '{}' not supported at line {}", ch, self.line)),
                }
                7 => match ch {
                    '|' => {
                        self.pos += 1;
                        return Token::And;
                    }
                    _ => return Token::Error(format!("Character '{}' not supported at line {}", ch, self.line)),
                }
                _ => return Token::Error(format!("Unknown state {}", self.state)),
            }
        }
        
        self.finalize_token(value)
    }

    fn finalize_token(&self, value: String) -> Token {
        match self.state {
            0 => return Token::EOF,
            1 => {
                match value.as_str() {
                    "var" => Token::KeywordVar,
                    "const" => Token::KeywordConst,

                    "if" => Token::KeywordIf,
                    "elif" => Token::KeywordElif,
                    "else" => Token::KeywordElse,

                    "print" => Token::KeywordPrint,
                    "println" => Token::KeywordPrintln,

                    "true" => Token::Boolean(true),
                    "false" => Token::Boolean(false),
                    _ => Token::Identifier(value),
                }
            }
            2 => {
                let n = value.parse::<i64>().expect("Failed to parse integer");
                return Token::Integer(n);
            }
            3 => {
                let string = value;
                return Token::String(string);
            }
            _ => return Token::Error(format!("Unknown state {}", self.state)),
        }
    }
}