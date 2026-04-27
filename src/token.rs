#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,
    KeywordPrint,
    
    LeftBracket,
    RightBracket,

    Identifier(String),
    Integer(i64),

    Assign,

    Semicolon,
    EOF,

    Error(String),
}