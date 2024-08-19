use std::fmt::Display;

use logos::{Lexer, Logos};

#[derive(Clone, Debug, Logos, PartialEq, Eq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    /*
     * Arithmetic operations
     */
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("==")]
    DoubleEq,
    #[token("!=")]
    NotEq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    /*
     * Language tokens
     */
    #[token("=")]
    Eq,
    #[token("->")]
    ThinArrow,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token(",")]
    Comma,

    /*
     * Matched tokens
     */
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    /*
     * Keywords
     */
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("break")]
    Break,

    /*
     * Primitive type
     */
    #[token("int")]
    Int,
    #[token("bool")]
    Bool,

    /*
     * Literals
     */
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, Token::parse_string)]
    String(String),

    #[regex(r#"\d+"#, Token::parse_integer)]
    Integer(i64),

    #[regex(r#"[a-zA-Z_]\w*"#, Token::parse_ident, priority = 1)]
    Ident(String),
}

impl Token {
    fn parse_string(lex: &mut Lexer<'_, Token>) -> String {
        lex.slice().to_owned()
    }

    fn parse_integer(lex: &mut Lexer<'_, Token>) -> i64 {
        lex.slice().to_owned().parse().unwrap()
    }

    fn parse_ident(lex: &mut Lexer<'_, Token>) -> String {
        lex.slice().to_owned()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::DoubleEq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Eq => write!(f, "="),
            Token::ThinArrow => write!(f, "->"),
            Token::Colon => write!(f, ":"),
            Token::SemiColon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::LeftParen => write!(f, ")"),
            Token::RightParen => write!(f, "("),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Loop => write!(f, "loop"),
            Token::Break => write!(f, "break"),
            Token::Int => write!(f, "int"),
            Token::Bool => write!(f, "bool"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::String(value) => write!(f, r#""{value}""#),
            Token::Integer(value) => write!(f, "{value}"),
            Token::Ident(value) => write!(f, "{value}"),
        }
    }
}
