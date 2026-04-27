#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    KeywordVar,
    
    Identifier(String),
    Integer(i64),

    Assign,

    Semicolon,
    EOF,
}