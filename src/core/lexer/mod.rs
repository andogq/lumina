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
    pub fn next_token(&mut self) -> Token {
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

            c if c.is_ascii_digit() => {
                let (mut literal, str_span) = self.source.consume_while(|c| c.is_ascii_digit());

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

            c => Token::Illegal(token::IllegalToken { span, c }),
        };

        self.consume_whitespace();

        token
    }

    pub fn peek(&mut self) -> Token {
        let token = self.next_token();
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

    fn test_tokens(source: &'static str, expected: &[Token]) {
        let mut lexer = Lexer::new(Source::new(source));

        let tokens = std::iter::from_fn(|| match lexer.next_token() {
            Token::EOF(_) => None,
            token => Some(token),
        })
        .collect::<Vec<_>>();

        assert_eq!(tokens, expected);
    }

    #[test]
    fn empty() {
        test_tokens("", &[]);
    }

    #[test]
    fn single_digit_integer() {
        test_tokens("1", &[Token::integer("1")]);
    }

    #[test]
    fn multi_digit_integer() {
        test_tokens("123", &[Token::integer("123")]);
    }

    #[test]
    fn multiple_integers() {
        test_tokens(
            "123 456 999",
            &[
                Token::integer("123"),
                Token::integer("456"),
                Token::integer("999"),
            ],
        );
    }

    #[test]
    fn single_character_ident() {
        test_tokens("a", &[Token::ident("a")]);
    }

    #[test]
    fn multi_character_ident() {
        test_tokens("asdf", &[Token::ident("asdf")]);
    }

    #[test]
    fn mulitple_idents() {
        test_tokens(
            "asdf some ident",
            &[
                Token::ident("asdf"),
                Token::ident("some"),
                Token::ident("ident"),
            ],
        );
    }

    #[test]
    fn token_plus() {
        test_tokens("+", &[Token::plus()]);
    }

    #[test]
    fn token_equals() {
        test_tokens("=", &[Token::equals()]);
    }

    #[test]
    fn token_semicolon() {
        test_tokens(";", &[Token::semicolon()]);
    }

    #[test]
    fn token_thin_arrow() {
        test_tokens("->", &[Token::thin_arrow()]);
    }

    #[test]
    fn token_left_paren() {
        test_tokens("(", &[Token::left_paren()]);
    }

    #[test]
    fn token_right_paren() {
        test_tokens(")", &[Token::right_paren()]);
    }

    #[test]
    fn token_left_brace() {
        test_tokens("{", &[Token::left_brace()]);
    }

    #[test]
    fn token_right_brace() {
        test_tokens("}", &[Token::right_brace()]);
    }

    #[test]
    fn token_true() {
        test_tokens("true", &[Token::t_true()]);
    }

    #[test]
    fn token_false() {
        test_tokens("false", &[Token::t_false()]);
    }

    #[test]
    fn token_fn() {
        test_tokens("fn", &[Token::t_fn()]);
    }

    #[test]
    fn token_return() {
        test_tokens("return", &[Token::t_return()]);
    }

    #[test]
    fn token_let() {
        test_tokens("let", &[Token::t_let()]);
    }

    #[test]
    fn token_if() {
        test_tokens("if", &[Token::t_if()]);
    }

    #[test]
    fn token_else() {
        test_tokens("else", &[Token::t_else()]);
    }

    #[test]
    fn ident_trueish() {
        test_tokens("trueish", &[Token::ident("trueish")]);
    }

    #[test]
    fn ident_falseish() {
        test_tokens("falseish", &[Token::ident("falseish")]);
    }

    #[test]
    fn ident_fnish() {
        test_tokens("fnish", &[Token::ident("fnish")]);
    }

    #[test]
    fn ident_returnish() {
        test_tokens("returnish", &[Token::ident("returnish")]);
    }

    #[test]
    fn ident_letish() {
        test_tokens("letish", &[Token::ident("letish")]);
    }

    #[test]
    fn ident_ifish() {
        test_tokens("ifish", &[Token::ident("ifish")]);
    }

    #[test]
    fn ident_elseish() {
        test_tokens("elseish", &[Token::ident("elseish")]);
    }

    #[test]
    fn simple_addition() {
        test_tokens(
            "1 + 3;",
            &[
                Token::integer("1"),
                Token::plus(),
                Token::integer("3"),
                Token::semicolon(),
            ],
        );
    }

    #[test]
    fn all_tokens() {
        test_tokens(
            "123 ident + = ; -> ( ) { } true false fn return let if else",
            &[
                Token::integer("123"),
                Token::ident("ident"),
                Token::plus(),
                Token::equals(),
                Token::semicolon(),
                Token::thin_arrow(),
                Token::left_paren(),
                Token::right_paren(),
                Token::left_brace(),
                Token::right_brace(),
                Token::t_true(),
                Token::t_false(),
                Token::t_fn(),
                Token::t_return(),
                Token::t_let(),
                Token::t_if(),
                Token::t_else(),
            ],
        );
    }
}
