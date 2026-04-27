#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    KeywordVar,

    Identifier(String),
    Integer(i64),

    Assign,

    Semicolon,
    EOF,

    Error(String),
}