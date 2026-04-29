#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,
    KeywordConst,

    KeywordPrint,
    KeywordPrintln,
    
    LeftBracket,
    RightBracket,
    
    Identifier(String),

    Integer(i64),

    String(String),

    Assign,

    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    Semicolon,
    EOF,

    Error(String),
}