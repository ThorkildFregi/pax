#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    KeywordVar,
    KeywordConst,
    KeywordGlobal,

    KeywordAppend,
    KeywordPop,

    KeywordIf,
    KeywordElif,
    KeywordElse,

    KeywordFor,
    KeywordIn,
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
    Dot,
    Colon,

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