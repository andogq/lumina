use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let Some(c) = self.input.next() else {
            return Token::EOF;
        };

        let token = match c {
            '=' => {
                let op = match self.input.peek() {
                    Some('=') => Some(Token::Eq),
                    _ => None,
                };

                if let Some(op) = op {
                    self.input.next().expect("character from previous peek");
                    op
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                let op = match self.input.peek() {
                    Some('=') => Some(Token::NotEq),
                    _ => None,
                };

                if let Some(op) = op {
                    self.input.next().expect("character from previous peek");
                    op
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LeftAngle,
            '>' => Token::RightAngle,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            c if c.is_ascii_alphabetic() || c == '_' => {
                // Continue to read the identifier
                let mut ident = String::from(c);
                self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_', Some(&mut ident));

                // Make sure the ident isn't a key word
                match ident.as_ref() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                }
            }
            c if c.is_ascii_digit() => {
                let mut int = String::from(c);
                self.consume_while(|c| c.is_ascii_digit(), Some(&mut int));

                Token::Int(int)
            }
            _ => Token::Illegal,
        };

        self.consume_whitespace();

        token
    }

    pub fn consume_while(&mut self, condition: fn(char) -> bool, mut value: Option<&mut String>) {
        while self.input.peek().map(|&c| condition(c)).unwrap_or_default() {
            let c = self.input.next().expect("character from previous peek");

            if let Some(value) = &mut value {
                (*value).push(c);
            }
        }
    }

    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace, None);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Token;

    #[test]
    fn next_token_simple() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        [
            Token::Assign,
            Token::Plus,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
        ]
        .into_iter()
        .for_each(|token| {
            assert_eq!(lexer.next_token(), token);
        });
    }

    #[test]
    fn next_token_full() {
        let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
          return true;
        } else {
          return false;
        }

        10 == 10;
        10 != 9;
        "#;
        let mut lexer = Lexer::new(input);

        [
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LeftParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RightParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::LeftAngle,
            Token::Int("10".to_string()),
            Token::RightAngle,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Int("5".to_string()),
            Token::LeftAngle,
            Token::Int("10".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,
            Token::Int("10".to_string()),
            Token::Eq,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::NotEq,
            Token::Int("9".to_string()),
            Token::Semicolon,
            Token::EOF,
        ]
        .into_iter()
        .for_each(|token| {
            assert_eq!(lexer.next_token(), token);
        });
    }
}
