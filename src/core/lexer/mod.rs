pub mod token;

use crate::util::source::Source;

use self::token::*;

pub struct Lexer {
    source: Source,

    next: Option<Token>,
}

impl Lexer {
    /// Create a new lexer with the provided source.
    pub fn new(source: Source) -> Self {
        let mut lexer = Self { source, next: None };

        // Consume all whitespace at the start of the file
        lexer.consume_whitespace();

        lexer
    }

    /// Get the next token. Will continually produce [`EOFToken`] if no more tokens can be
    /// produced.
    pub fn next(&mut self) -> Token {
        if let Some(token) = self.next.take() {
            return token;
        }

        // Fetch the next character
        let Some((c, location)) = self.source.next() else {
            return Token::EOF(EOFToken {
                span: self.source.location().span(),
            });
        };

        // Pre-emptively create a span for this character
        let span = location.clone().span();

        let token = match c {
            '+' => Token::Plus(PlusToken { span }),
            '=' => Token::Equals(EqualsToken { span }),

            '(' => Token::LeftParen(LeftParenToken { span }),
            ')' => Token::RightParen(RightParenToken { span }),
            '{' => Token::LeftBrace(LeftBraceToken { span }),
            '}' => Token::RightBrace(RightBraceToken { span }),

            '-' if matches!(self.source.peek(), Some('>')) => {
                let (_, end_location) = self.source.next().expect("arrow close");

                Token::ThinArrow(ThinArrowToken {
                    span: (location, end_location).into(),
                })
            }

            ';' => Token::Semicolon(SemicolonToken { span }),

            c if c.is_digit(10) => {
                let (mut literal, str_span) = self.source.consume_while(|c| c.is_digit(10));

                // Add the first digit of the number
                literal.insert(0, c);

                // Expand the span to include the first digit
                let span = span.to(&str_span);

                Token::Integer(IntegerToken { span, literal })
            }

            c if c.is_alphabetic() => {
                let (mut literal, str_span) = self.source.consume_while(|c| c.is_alphabetic());

                literal.insert(0, c);

                let span = span.to(&str_span);

                match literal.as_str() {
                    "true" => Token::True(TrueToken { span }),
                    "false" => Token::False(FalseToken { span }),
                    "fn" => Token::Fn(FnToken { span }),
                    "return" => Token::Return(ReturnToken { span }),
                    "let" => Token::Let(LetToken { span }),
                    "if" => Token::If(IfToken { span }),
                    "else" => Token::Else(ElseToken { span }),
                    _ => Token::Ident(IdentToken { span, literal }),
                }
            }

            _ => Token::Illegal(token::IllegalToken { span }),
        };

        self.consume_whitespace();

        token
    }

    pub fn peek(&mut self) -> Token {
        let token = self.next();
        self.next = Some(token.clone());
        token
    }

    /// Consume the underlying source whilst whitespace is detected.
    fn consume_whitespace(&mut self) {
        self.source.consume_while(|c| c.is_whitespace());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_addition() {
        let mut lexer = Lexer::new(Source::new("1 + 3;"));

        let tokens = std::iter::from_fn(|| match lexer.next() {
            Token::EOF(_) => None,
            token => Some(token),
        })
        .collect::<Vec<_>>();

        assert_eq!(
            tokens,
            [
                Token::Integer(IntegerToken {
                    literal: "1".to_string(),
                    ..Default::default()
                }),
                Token::Plus(Default::default()),
                Token::Integer(IntegerToken {
                    literal: "3".to_string(),
                    ..Default::default()
                }),
                Token::Semicolon(Default::default()),
            ]
        );
    }
}
