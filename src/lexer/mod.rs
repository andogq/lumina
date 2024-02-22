use std::{iter::Peekable, str::Chars};

use crate::token::{
    AssignToken, AsteriskToken, BangToken, CommaToken, EOFToken, ElseToken, EqToken, FalseToken,
    FunctionToken, IdentToken, IfToken, IllegalToken, IntToken, LeftAngleToken, LeftBraceToken,
    LeftParenToken, LetToken, MinusToken, NotEqToken, PlusToken, ReturnToken, RightAngleToken,
    RightBraceToken, RightParenToken, SemicolonToken, SlashToken, Token, TrueToken,
};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    end: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            end: false,
        }
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

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let Some(c) = self.input.next() else {
            self.end = true;
            return Some(Token::EOF(EOFToken));
        };

        let token = match c {
            '=' => {
                let op = match self.input.peek() {
                    Some('=') => Some(Token::Eq(EqToken)),
                    _ => None,
                };

                if let Some(op) = op {
                    self.input.next().expect("character from previous peek");
                    op
                } else {
                    Token::Assign(AssignToken)
                }
            }
            '+' => Token::Plus(PlusToken),
            '-' => Token::Minus(MinusToken),
            '!' => {
                let op = match self.input.peek() {
                    Some('=') => Some(Token::NotEq(NotEqToken)),
                    _ => None,
                };

                if let Some(op) = op {
                    self.input.next().expect("character from previous peek");
                    op
                } else {
                    Token::Bang(BangToken)
                }
            }
            '*' => Token::Asterisk(AsteriskToken),
            '/' => Token::Slash(SlashToken),
            '<' => Token::LeftAngle(LeftAngleToken),
            '>' => Token::RightAngle(RightAngleToken),
            ';' => Token::Semicolon(SemicolonToken),
            ',' => Token::Comma(CommaToken),
            '(' => Token::LeftParen(LeftParenToken),
            ')' => Token::RightParen(RightParenToken),
            '{' => Token::LeftBrace(LeftBraceToken),
            '}' => Token::RightBrace(RightBraceToken),
            c if c.is_ascii_alphabetic() || c == '_' => {
                // Continue to read the identifier
                let mut ident = String::from(c);
                self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_', Some(&mut ident));

                // Make sure the ident isn't a key word
                match ident.as_ref() {
                    "fn" => Token::Function(FunctionToken),
                    "let" => Token::Let(LetToken),
                    "true" => Token::True(TrueToken),
                    "false" => Token::False(FalseToken),
                    "if" => Token::If(IfToken),
                    "else" => Token::Else(ElseToken),
                    "return" => Token::Return(ReturnToken),
                    _ => Token::Ident(IdentToken { literal: ident }),
                }
            }
            c if c.is_ascii_digit() => {
                let mut int = String::from(c);
                self.consume_while(|c| c.is_ascii_digit(), Some(&mut int));

                Token::Int(IntToken { literal: int })
            }
            _ => Token::Illegal(IllegalToken),
        };

        self.consume_whitespace();

        Some(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Token;

    #[test]
    fn next_token_simple() {
        let input = "=+(){},;";
        let lexer = Lexer::new(input);

        [
            Token::Assign(AssignToken),
            Token::Plus(PlusToken),
            Token::LeftParen(LeftParenToken),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::RightBrace(RightBraceToken),
            Token::Comma(CommaToken),
            Token::Semicolon(SemicolonToken),
        ]
        .into_iter()
        .zip(lexer)
        .for_each(|(expected, token)| {
            assert_eq!(expected, token);
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
        let lexer = Lexer::new(input);

        [
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "five".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "ten".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Int(IntToken {
                literal: "10".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "add".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Function(FunctionToken),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
            }),
            Token::Plus(PlusToken),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::RightBrace(RightBraceToken),
            Token::Semicolon(SemicolonToken),
            Token::Let(LetToken),
            Token::Ident(IdentToken {
                literal: "result".to_string(),
            }),
            Token::Assign(AssignToken),
            Token::Ident(IdentToken {
                literal: "add".to_string(),
            }),
            Token::LeftParen(LeftParenToken),
            Token::Ident(IdentToken {
                literal: "five".to_string(),
            }),
            Token::Comma(CommaToken),
            Token::Ident(IdentToken {
                literal: "ten".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::Semicolon(SemicolonToken),
            Token::Bang(BangToken),
            Token::Minus(MinusToken),
            Token::Slash(SlashToken),
            Token::Asterisk(AsteriskToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::LeftAngle(LeftAngleToken),
            Token::Int(IntToken {
                literal: "10".to_string(),
            }),
            Token::RightAngle(RightAngleToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::If(IfToken),
            Token::LeftParen(LeftParenToken),
            Token::Int(IntToken {
                literal: "5".to_string(),
            }),
            Token::LeftAngle(LeftAngleToken),
            Token::Int(IntToken {
                literal: "10".to_string(),
            }),
            Token::RightParen(RightParenToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Return(ReturnToken),
            Token::True(TrueToken),
            Token::Semicolon(SemicolonToken),
            Token::RightBrace(RightBraceToken),
            Token::Else(ElseToken),
            Token::LeftBrace(LeftBraceToken),
            Token::Return(ReturnToken),
            Token::False(FalseToken),
            Token::Semicolon(SemicolonToken),
            Token::RightBrace(RightBraceToken),
            Token::Int(IntToken {
                literal: "10".to_string(),
            }),
            Token::Eq(EqToken),
            Token::Int(IntToken {
                literal: "10".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::Int(IntToken {
                literal: "10".to_string(),
            }),
            Token::NotEq(NotEqToken),
            Token::Int(IntToken {
                literal: "9".to_string(),
            }),
            Token::Semicolon(SemicolonToken),
            Token::EOF(EOFToken),
        ]
        .into_iter()
        .zip(lexer)
        .for_each(|(expected, token)| {
            assert_eq!(expected, token);
        });
    }
}
