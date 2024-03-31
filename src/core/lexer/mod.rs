pub mod token;

use crate::util::source::Source;

use self::token::{EOFToken, IntegerToken, PlusToken, SemicolonToken, Token};

pub struct Lexer<S>
where
    S: Iterator,
{
    source: Source<S>,

    next: Option<Token>,
}

impl<S> Lexer<S>
where
    S: Iterator<Item = char>,
{
    /// Create a new lexer with the provided source.
    pub fn new(source: Source<S>) -> Self {
        Self { source, next: None }
    }

    /// Get the next token. Will continually produce [`EOFToken`] if no more tokens can be
    /// produced.
    pub fn next(&mut self) -> Token {
        if let Some(token) = self.next.take() {
            return token;
        }

        // Track the start of the span
        let span_start = self.source.location();

        // Pre-emptively create a span for this character
        let span = self.source.span(span_start.clone());

        // Fetch the next character
        let Some(c) = self.source.next() else {
            return Token::EOF(EOFToken { span });
        };

        let token = match c {
            '+' => Token::Plus(PlusToken { span }),

            ';' => Token::Semicolon(SemicolonToken { span }),

            c if c.is_digit(10) => {
                let integer = {
                    let mut s = self.source.consume_while(|c| c.is_digit(10));
                    s.insert(0, c);
                    s
                };

                Token::Integer(IntegerToken {
                    span: self.source.span(span_start),
                    literal: integer,
                })
            }

            _ => Token::Illegal(token::IllegalToken {
                span: self.source.span(span_start),
            }),
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
        let mut lexer = Lexer::new(Source::new("test", "1 + 3;".chars()));

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
