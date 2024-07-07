pub mod token;

use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use crate::util::source::Source;

use self::token::*;

pub enum TokenIter {
    Source(Source),
    Vec(std::vec::IntoIter<Token>),
}

impl TokenIter {
    /// Create a new lexer with the provided source.
    pub fn new(source: Source) -> Self {
        Self::Source(source)
    }

    pub fn new_vec(vec: Vec<Token>) -> Self {
        Self::Vec(vec.into_iter())
    }
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TokenIter::Source(source) => {
                // Consume whitespace
                source.consume_while(|c| c.is_whitespace());

                // Fetch the next character
                let (c, location) = source.next()?;

                // Pre-emptively create a span for this character
                let span = location.clone().span();

                let token = match c {
                    '+' => Token::Plus(PlusToken { span }),
                    '=' if matches!(source.peek(), Some('=')) => {
                        let (_, end_location) = source.next().expect("eq second =");

                        Token::Eq(EqToken {
                            span: (location, end_location).into(),
                        })
                    }
                    '!' if matches!(source.peek(), Some('=')) => {
                        let (_, end_location) = source.next().expect("not eq second =");

                        Token::NotEq(NotEqToken {
                            span: (location, end_location).into(),
                        })
                    }

                    '=' => Token::Equals(EqualsToken { span }),

                    '(' => Token::LeftParen(LeftParenToken { span }),
                    ')' => Token::RightParen(RightParenToken { span }),
                    '{' => Token::LeftBrace(LeftBraceToken { span }),
                    '}' => Token::RightBrace(RightBraceToken { span }),

                    '-' if matches!(source.peek(), Some('>')) => {
                        let (_, end_location) = source.next().expect("arrow close");

                        Token::ThinArrow(ThinArrowToken {
                            span: (location, end_location).into(),
                        })
                    }

                    ';' => Token::Semicolon(SemicolonToken { span }),
                    ',' => Token::Comma(CommaToken { span }),

                    c if c.is_ascii_digit() => {
                        let (mut literal, str_span) = source.consume_while(|c| c.is_ascii_digit());

                        // Add the first digit of the number
                        literal.insert(0, c);

                        // Expand the span to include the first digit
                        let span = span.to(&str_span);

                        Token::Integer(IntegerToken { span, literal })
                    }

                    c if c.is_alphabetic() => {
                        let (mut literal, str_span) = source.consume_while(|c| c.is_alphabetic());

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

                Some(token)
            }
            TokenIter::Vec(vec) => vec.next(),
        }
    }
}

pub struct Lexer(Peekable<TokenIter>);

impl Lexer {
    pub fn new(source: Source) -> Self {
        Self(TokenIter::new(source).peekable())
    }

    pub fn with_tokens(tokens: Vec<Token>) -> Self {
        Self(TokenIter::new_vec(tokens).peekable())
    }

    /// Retrieve the next token, or [`Token::EOF`] if no more tokens follow.
    pub fn next_token(&mut self) -> Token {
        // WARN: Not sure if the span matters for this token
        self.0.next().unwrap_or(Token::EOF(Default::default()))
    }

    /// Peek the next token, or [`Token::EOF`] if no more tokens follow.
    pub fn peek_token(&mut self) -> Token {
        self.0
            .peek()
            .cloned()
            // WARN: Not sure if the span matters for this token
            .unwrap_or(Token::EOF(Default::default()))
    }
}

impl IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = Peekable<TokenIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}

impl Deref for Lexer {
    type Target = Peekable<TokenIter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty("", &[])]
    #[case::single_digit_integer("1", &[Token::integer("1")])]
    #[case::multi_digit_integer("123", &[Token::integer("123")])]
    #[case::multi_integer("123 456 999", &[Token::integer("123"), Token::integer("456"), Token::integer("999")])]
    #[case::single_char_ident("a", &[Token::ident("a")])]
    #[case::multi_char_ident("asdf", &[Token::ident("asdf")])]
    #[case::multi_ident("asdf some ident", &[Token::ident("asdf"), Token::ident("some"), Token::ident("ident")])]
    #[case::plus("+", &[Token::plus()])]
    #[case::eq("==", &[Token::eq()])]
    #[case::not_eq("!=", &[Token::not_eq()])]
    #[case::equals("=", &[Token::equals()])]
    #[case::semicolon(";", &[Token::semicolon()])]
    #[case::thin_arrow("->", &[Token::thin_arrow()])]
    #[case::comma(",", &[Token::comma()])]
    #[case::left_paren("(", &[Token::left_paren()])]
    #[case::right_paren(")", &[Token::right_paren()])]
    #[case::left_brace("{", &[Token::left_brace()])]
    #[case::right_brace("}", &[Token::right_brace()])]
    #[case::t_true("true", &[Token::t_true()])]
    #[case::t_false("false", &[Token::t_false()])]
    #[case::t_fn("fn", &[Token::t_fn()])]
    #[case::t_return("return", &[Token::t_return()])]
    #[case::t_let("let", &[Token::t_let()])]
    #[case::t_if("if", &[Token::t_if()])]
    #[case::t_else("else", &[Token::t_else()])]
    #[case::trueish("trueish", &[Token::ident("trueish")])]
    #[case::falseish("falseish", &[Token::ident("falseish")])]
    #[case::fnish("fnish", &[Token::ident("fnish")])]
    #[case::returnish("returnish", &[Token::ident("returnish")])]
    #[case::letish("letish", &[Token::ident("letish")])]
    #[case::ifish("ifish", &[Token::ident("ifish")])]
    #[case::elseish("elseish", &[Token::ident("elseish")])]
    #[case::simple_addition("1 + 3;", &[
        Token::integer("1"),
        Token::plus(),
        Token::integer("3"),
        Token::semicolon(),
    ])]
    #[case::all_tokens(
        "123 ident + == != = ; -> , ( ) { } true false fn return let if else",
        &[
            Token::integer("123"),
            Token::ident("ident"),
            Token::plus(),
            Token::eq(),
            Token::not_eq(),
            Token::equals(),
            Token::semicolon(),
            Token::thin_arrow(),
            Token::comma(),
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
    )]
    fn test_tokens(#[case] source: &'static str, #[case] expected: &[Token]) {
        let lexer = Lexer::new(Source::new(source));
        let tokens = Vec::from_iter(lexer.into_iter());

        assert_eq!(tokens, expected);
    }
}
