#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,
    KeywordPrint,
    KeywordPrintln,
    
    LeftBracket,
    RightBracket,

    Identifier(String),
    Integer(i64),

    Assign,

    Plus,
    Minus,

    Semicolon,
    EOF,

    Error(String),
}