#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    // Literals
    Identifier(String),
    Int(String),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrackets,
    RightBrackets,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
