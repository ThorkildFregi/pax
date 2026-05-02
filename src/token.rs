#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,
    KeywordConst,
    KeywordGlobal,

    KeywordIf,
    KeywordElif,
    KeywordElse,

    KeywordWhile,

    KeywordPrint,
    KeywordPrintln,
    
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
    
    Identifier(String),

    Integer(i64),

    String(String),

    Boolean(bool),

    Assign,
    Comma,

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