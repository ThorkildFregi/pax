use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    state: u32,
    line: u32
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
                        self.pos += 1;
                        return Token::Assign;
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
                    _ => Token::Identifier(value),
                }
            }
            2 => {
                let n = value.parse::<i64>().expect("Failed to parse integer");
                return Token::Integer(n);
            }
            _ => return Token::Error(format!("Unknown state {}", self.state)),
        }
    }
}