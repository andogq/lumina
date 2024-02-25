use std::collections::VecDeque;

use crate::token::*;

pub use self::source::Source;

mod source;

pub struct Lexer<S> {
    source: Source<S>,

    buffer: VecDeque<Token>,

    end: bool,
}

impl<S> Lexer<S>
where
    S: Iterator<Item = char>,
{
    /// Create a new lexer with the provided source.
    pub fn new(source: Source<S>) -> Self {
        Self {
            source,
            buffer: VecDeque::new(),
            end: false,
        }
    }

    fn next_token(&mut self) -> Token {
        // Initialise the span
        let span_start = self.source.location();

        // Continually emit EOF if no more characters left in the source
        let Some(c) = self.source.next() else {
            return Token::EOF(EOFToken {
                span: self.source.span(span_start),
            });
        };

        let token = match c {
            '=' => {
                let op = match self.source.peek() {
                    Some('=') => Some(Token::Eq(EqToken {
                        span: self.source.span(span_start.clone()),
                    })),
                    _ => None,
                };

                if let Some(op) = op {
                    self.source.next().expect("character from previous peek");
                    op
                } else {
                    Token::Assign(AssignToken {
                        span: self.source.span(span_start),
                    })
                }
            }
            '+' => Token::Plus(PlusToken {
                span: self.source.span(span_start),
            }),
            '-' => Token::Minus(MinusToken {
                span: self.source.span(span_start),
            }),
            '!' => {
                let op = match self.source.peek() {
                    Some('=') => Some(Token::NotEq(NotEqToken {
                        span: self.source.span(span_start.clone()),
                    })),
                    _ => None,
                };

                if let Some(op) = op {
                    self.source.next().expect("character from previous peek");
                    op
                } else {
                    Token::Bang(BangToken {
                        span: self.source.span(span_start),
                    })
                }
            }
            '*' => Token::Asterisk(AsteriskToken {
                span: self.source.span(span_start),
            }),
            '/' => Token::Slash(SlashToken {
                span: self.source.span(span_start),
            }),
            '<' => Token::LeftAngle(LeftAngleToken {
                span: self.source.span(span_start),
            }),
            '>' => Token::RightAngle(RightAngleToken {
                span: self.source.span(span_start),
            }),
            ';' => Token::Semicolon(SemicolonToken {
                span: self.source.span(span_start),
            }),
            ',' => Token::Comma(CommaToken {
                span: self.source.span(span_start),
            }),
            '(' => Token::LeftParen(LeftParenToken {
                span: self.source.span(span_start),
            }),
            ')' => Token::RightParen(RightParenToken {
                span: self.source.span(span_start),
            }),
            '{' => Token::LeftBrace(LeftBraceToken {
                span: self.source.span(span_start),
            }),
            '}' => Token::RightBrace(RightBraceToken {
                span: self.source.span(span_start),
            }),
            '"' => {
                // Read through the string
                let mut s = String::new();
                while self.source.peek().map(|c| c != '"').unwrap_or(false) {
                    let Some(c) = self.source.next() else {
                        break;
                    };

                    // If next character is escaped, skip forward and add it instead
                    if c == '\\' {
                        let Some(c) = self.source.next() else {
                            break;
                        };

                        s.push(c);
                    } else {
                        s.push(c)
                    }
                }

                // Eat the closing quote
                self.source.next();

                Token::String(StringToken {
                    span: self.source.span(span_start),
                    literal: s,
                })
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                // Continue to read the identifier
                let ident = {
                    let mut ident = self
                        .source
                        .consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
                    ident.insert(0, c);
                    ident
                };

                // Make sure the ident isn't a key word
                match ident.as_ref() {
                    "fn" => Token::Function(FunctionToken {
                        span: self.source.span(span_start),
                    }),
                    "let" => Token::Let(LetToken {
                        span: self.source.span(span_start),
                    }),
                    "true" => Token::True(TrueToken {
                        span: self.source.span(span_start),
                    }),
                    "false" => Token::False(FalseToken {
                        span: self.source.span(span_start),
                    }),
                    "if" => Token::If(IfToken {
                        span: self.source.span(span_start),
                    }),
                    "else" => Token::Else(ElseToken {
                        span: self.source.span(span_start),
                    }),
                    "return" => Token::Return(ReturnToken {
                        span: self.source.span(span_start),
                    }),
                    _ => Token::Ident(IdentToken {
                        span: self.source.span(span_start),
                        literal: ident,
                    }),
                }
            }
            c if c.is_ascii_digit() => {
                let int = {
                    let mut s = self.source.consume_while(|c| c.is_ascii_digit());
                    // Prepend literal with first int
                    s.insert(0, c);
                    s
                };

                Token::Int(IntToken {
                    span: self.source.span(span_start),
                    literal: int,
                })
            }
            _ => Token::Illegal(IllegalToken {
                span: self.source.span(span_start),
            }),
        };

        self.source.consume_while(|c| c.is_whitespace());

        token
    }

    pub fn next(&mut self) -> Token {
        let token = self.buffer.pop_front().unwrap_or_else(|| self.next_token());

        if matches!(token, Token::EOF(_)) {
            self.end = true;
        }

        token
    }

    pub fn peek_ahead(&mut self, offset: usize) -> Token {
        while self.buffer.len() < offset {
            match self.next_token() {
                token @ Token::EOF(_) => return token,
                token => self.buffer.push_back(token),
            }
        }

        offset
            .checked_sub(1)
            .and_then(|offset| self.buffer.get(offset))
            .cloned()
            .unwrap_or_else(|| self.next_token())
    }

    pub fn peek(&mut self) -> Token {
        self.peek_ahead(1)
    }

    pub fn next_if(&mut self, condition: fn(Token) -> bool) -> Option<Token> {
        if condition(self.peek()) {
            Some(self.next())
        } else {
            None
        }
    }
}

impl Lexer<std::array::IntoIter<char, 0>> {
    pub fn from_tokens(tokens: impl IntoIterator<Item = Token>) -> Self {
        Self {
            source: Source::new("", []),
            buffer: VecDeque::from_iter(tokens.into_iter()),
            end: false,
        }
    }
}

impl<S> Iterator for Lexer<S>
where
    S: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            None
        } else {
            Some(Lexer::next(self))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Token;

    macro_rules! lexer {
        ($input:expr) => {
            Lexer::new(Source::new("test", $input.chars()))
        };
    }

    #[test]
    fn next_token_simple() {
        let lexer = lexer!("=+(){},;");

        [
            Token::Assign(AssignToken::default()),
            Token::Plus(PlusToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::RightBrace(RightBraceToken::default()),
            Token::Comma(CommaToken::default()),
            Token::Semicolon(SemicolonToken::default()),
        ]
        .into_iter()
        .zip(lexer)
        .for_each(|(expected, token)| {
            assert_eq!(expected, token);
        });
    }

    #[test]
    fn next_token_full() {
        let lexer = lexer!(
            r#"let five = 5;
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
            10 != 9;"#
        );

        [
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "five".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "ten".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Int(IntToken {
                literal: "10".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "add".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Function(FunctionToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Ident(IdentToken {
                literal: "x".to_string(),
                ..Default::default()
            }),
            Token::Plus(PlusToken::default()),
            Token::Ident(IdentToken {
                literal: "y".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::RightBrace(RightBraceToken::default()),
            Token::Semicolon(SemicolonToken::default()),
            Token::Let(LetToken::default()),
            Token::Ident(IdentToken {
                literal: "result".to_string(),
                ..Default::default()
            }),
            Token::Assign(AssignToken::default()),
            Token::Ident(IdentToken {
                literal: "add".to_string(),
                ..Default::default()
            }),
            Token::LeftParen(LeftParenToken::default()),
            Token::Ident(IdentToken {
                literal: "five".to_string(),
                ..Default::default()
            }),
            Token::Comma(CommaToken::default()),
            Token::Ident(IdentToken {
                literal: "ten".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::Semicolon(SemicolonToken::default()),
            Token::Bang(BangToken::default()),
            Token::Minus(MinusToken::default()),
            Token::Slash(SlashToken::default()),
            Token::Asterisk(AsteriskToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::LeftAngle(LeftAngleToken::default()),
            Token::Int(IntToken {
                literal: "10".to_string(),
                ..Default::default()
            }),
            Token::RightAngle(RightAngleToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::If(IfToken::default()),
            Token::LeftParen(LeftParenToken::default()),
            Token::Int(IntToken {
                literal: "5".to_string(),
                ..Default::default()
            }),
            Token::LeftAngle(LeftAngleToken::default()),
            Token::Int(IntToken {
                literal: "10".to_string(),
                ..Default::default()
            }),
            Token::RightParen(RightParenToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Return(ReturnToken::default()),
            Token::True(TrueToken::default()),
            Token::Semicolon(SemicolonToken::default()),
            Token::RightBrace(RightBraceToken::default()),
            Token::Else(ElseToken::default()),
            Token::LeftBrace(LeftBraceToken::default()),
            Token::Return(ReturnToken::default()),
            Token::False(FalseToken::default()),
            Token::Semicolon(SemicolonToken::default()),
            Token::RightBrace(RightBraceToken::default()),
            Token::Int(IntToken {
                literal: "10".to_string(),
                ..Default::default()
            }),
            Token::Eq(EqToken::default()),
            Token::Int(IntToken {
                literal: "10".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::Int(IntToken {
                literal: "10".to_string(),
                ..Default::default()
            }),
            Token::NotEq(NotEqToken::default()),
            Token::Int(IntToken {
                literal: "9".to_string(),
                ..Default::default()
            }),
            Token::Semicolon(SemicolonToken::default()),
            Token::EOF(EOFToken::default()),
        ]
        .into_iter()
        .zip(lexer)
        .for_each(|(expected, token)| {
            assert_eq!(expected, token);
        });
    }

    #[test]
    fn parse_string() {
        let lexer = lexer!(r#""some string""#);

        [
            Token::String(StringToken {
                literal: "some string".to_string(),
                ..Default::default()
            }),
            Token::EOF(EOFToken::default()),
        ]
        .into_iter()
        .zip(lexer)
        .for_each(|(expected, token)| {
            assert_eq!(expected, token);
        });
    }

    #[test]
    fn parse_escaped_string() {
        let lexer = lexer!(r#""some \"string\"!""#);

        [
            Token::String(StringToken {
                literal: r#"some "string"!"#.to_string(),
                ..Default::default()
            }),
            Token::EOF(EOFToken::default()),
        ]
        .into_iter()
        .zip(lexer)
        .for_each(|(expected, token)| {
            assert_eq!(expected, token);
        });
    }
}
