#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,
    KeywordConst,
    KeywordGlobal,

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
    More,
    Less,
    MoreEqual,
    LessEqual,

    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    Semicolon,
    EOF,

    Error(String),
}