#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    /// Illegal/unknown token
    Illegal,
    /// End of file
    EOF,

    // Identifiers and literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LeftAngle,
    RightAngle,

    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    // Brackets
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
