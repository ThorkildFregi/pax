#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,
    KeywordConst,

    KeywordIf,
    KeywordElif,
    KeywordElse,

    KeywordPrint,
    KeywordPrintln,
    
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    
    Identifier(String),

    Integer(i64),

    String(String),

    Boolean(bool),

    Assign,

    And,
    Or,
    Not,

    Equal,
    Different,

    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    Semicolon,
    EOF,

    Error(String),
}