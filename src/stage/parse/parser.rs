// Temporary until this is integrated
#![allow(dead_code)]

use std::collections::HashMap;

use crate::{compiler::Compiler, repr::ast::untyped::Expression};

use super::{Lexer, ParseError, Token};

/// Function capable of parsing an infix expression out of the provided lexer.
type InfixParser = fn(
    parser: &Parser,
    compiler: &mut Compiler,
    lexer: &mut Lexer,
    left: Expression,
) -> Result<Expression, ParseError>;

/// Function capable of parsing a prefix expression out of the provided lexer.
type PrefixParser = fn(
    parser: &Parser,
    compiler: &mut Compiler,
    lexer: &mut Lexer,
) -> Result<Expression, ParseError>;

/// Function to test whether a token is a match to parse.
type TokenTest = fn(token: &Token) -> bool;

/// Composable parser, allowing for components of the parser to be dynamically registered.
struct Parser {
    /// Infix parse function to run when a given token is presented during infix parsing.
    infix: HashMap<Token, InfixParser>,
    /// Dynamic tests to run on a token during infix parsing. These will be run after the infix map is checked.
    infix_tests: Vec<(TokenTest, InfixParser)>,
    /// Prefix parse function to run when a given token is presented durign prefix parsing.
    prefix: HashMap<Token, PrefixParser>,
    /// Dynamic tests to run on a token during prefix parsing. These will be run after the prefix map is checked.
    prefix_tests: Vec<(TokenTest, PrefixParser)>,
}

impl Parser {
    /// Create a new instance of the parser.
    pub fn new() -> Self {
        Self {
            infix: HashMap::new(),
            infix_tests: Vec::new(),
            prefix: HashMap::new(),
            prefix_tests: Vec::new(),
        }
    }

    /// Register a new prefix parser against a token. Will return `false` if the token has already
    /// been registered.
    pub fn register_prefix(&mut self, token: Token, parser: PrefixParser) -> bool {
        // If the token has already been registered, bail
        if self.prefix.contains_key(&token) {
            return false;
        }

        self.prefix.insert(token, parser);

        true
    }

    /// Register a test for a prefix parser.
    pub fn register_prefix_test(&mut self, token_test: TokenTest, parser: PrefixParser) {
        self.prefix_tests.push((token_test, parser));
    }

    /// Register a new infix parser against a token. Will return `fasle` if the token has already
    /// been registered.
    pub fn register_infix(&mut self, token: Token, parser: InfixParser) -> bool {
        // If the token has already been registered, bail
        if self.infix.contains_key(&token) {
            return false;
        }

        self.infix.insert(token, parser);

        true
    }

    /// Register a test for an infix parser.
    pub fn register_infix_test(&mut self, token_test: TokenTest, parser: InfixParser) {
        self.infix_tests.push((token_test, parser));
    }

    /// Parse an expression starting with the given precedence out of the lexer.
    pub fn parse<P: From<Token> + PartialOrd>(
        &self,
        compiler: &mut Compiler,
        lexer: &mut Lexer,
        precedence: P,
    ) -> Result<Expression, ParseError> {
        // Parse a prefix for this expression
        let mut left = self.parse_prefix(compiler, lexer)?;

        while let Some(token) = lexer
            .peek_token()
            .filter(|t| precedence < P::from((*t).clone()))
        {
            let Some(parser) = self.get_infix_parser(token) else {
                // Can't find an infix parser for the next token, likely finished this expression
                return Ok(left);
            };

            left = parser(self, compiler, lexer, left)?;
        }

        Ok(left)
    }

    /// Parse a prefix expression
    fn parse_prefix(
        &self,
        compiler: &mut Compiler,
        lexer: &mut Lexer,
    ) -> Result<Expression, ParseError> {
        let token = lexer.peek_token().unwrap();

        let parser = self
            .get_prefix_parser(token)
            .ok_or_else(|| ParseError::UnexpectedToken(token.clone()))?;

        parser(self, compiler, lexer)
    }

    /// Attempt to find a prefix parser for a given token.
    fn get_prefix_parser<'a>(&'a self, token: &'a Token) -> Option<&'a PrefixParser> {
        self.get_parser(&self.prefix, &self.prefix_tests, token)
    }

    /// Attempt to find an infix parser for a given token.
    fn get_infix_parser<'a>(&'a self, token: &'a Token) -> Option<&'a InfixParser> {
        self.get_parser(&self.infix, &self.infix_tests, token)
    }

    /// Attempt to find a parser from a lookup and set of tests for a given token.
    fn get_parser<'a, P>(
        &'a self,
        lookup: &'a HashMap<Token, P>,
        tests: &'a [(TokenTest, P)],
        token: &'a Token,
    ) -> Option<&'a P> {
        lookup.get(token).or_else(|| {
            tests
                .iter()
                .find(|(test, _)| test(token))
                .map(|(_, parser)| parser)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        hir::{Boolean, Expression, Integer},
        stage::parse::Precedence,
    };

    use super::*;
    use rstest::*;

    /// Ensure parsing fails when no parsers are registered.
    #[rstest]
    fn parse_no_match() {
        let parser = Parser::new();

        let result = parser.parse(
            &mut Compiler::default(),
            &mut Lexer::from("true"),
            Precedence::Lowest,
        );

        assert!(matches!(result, Err(ParseError::UnexpectedToken(_))));
    }

    mod prefix {
        use super::*;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            parser.register_prefix(Token::True, |_, _, lexer| {
                let span = match lexer.next_spanned().unwrap() {
                    (Token::True, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::True),
                            found: Box::new(token),
                            reason: "expected true".to_string(),
                        });
                    }
                };

                Ok(Expression::Boolean(Boolean {
                    value: true,
                    span,
                    ty_info: None,
                }))
            });

            parser.register_prefix(Token::False, |_, _, lexer| {
                let span = match lexer.next_spanned().unwrap() {
                    (Token::False, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::False),
                            found: Box::new(token),
                            reason: "expected false".to_string(),
                        });
                    }
                };

                Ok(Expression::Boolean(Boolean {
                    value: false,
                    span,
                    ty_info: None,
                }))
            });

            parser.register_prefix_test(
                |token| matches!(token, Token::Integer(_)),
                |_, _, lexer| {
                    let (value, span) = match lexer.next_spanned().unwrap() {
                        (Token::Integer(value), span) => (value, span),
                        (token, _) => {
                            return Err(ParseError::ExpectedToken {
                                expected: Box::new(Token::Integer(1)),
                                found: Box::new(token),
                                reason: "expected integer".to_string(),
                            });
                        }
                    };

                    Ok(Expression::Integer(Integer {
                        value,
                        span,
                        ty_info: None,
                    }))
                },
            );

            parser
        }

        /// Ensure that a prefix parser can be correctly registered.
        #[rstest]
        fn register_prefix_parser() {
            let mut parser = Parser::new();

            let registered = parser.register_prefix(Token::True, |_, _, _| {
                Ok(Expression::Boolean(Boolean {
                    value: true,
                    span: Default::default(),
                    ty_info: None,
                }))
            });

            assert!(registered, "can add parser for new token");
        }

        /// Ensure that registration fails if a token has already been registered.
        #[rstest]
        fn fail_register_duplicate_prefix_parser() {
            let mut parser = Parser::new();

            let first_register = parser.register_prefix(Token::True, |_, _, _| {
                Ok(Expression::Boolean(Boolean {
                    value: true,
                    span: Default::default(),
                    ty_info: None,
                }))
            });

            assert!(first_register, "successfully register new token");

            let second_register = parser.register_prefix(Token::True, |_, _, _| {
                Ok(Expression::Boolean(Boolean {
                    value: true,
                    span: Default::default(),
                    ty_info: None,
                }))
            });

            assert!(!second_register, "cannot re-register for same token");
        }

        /// Ensure that a prefix test parser can be registered.
        #[rstest]
        fn register_prefix_test() {
            let mut parser = Parser::new();

            parser.register_prefix_test(
                |token| matches!(token, Token::Integer(_)),
                |_, _, _| {
                    Ok(Expression::Integer(Integer {
                        value: 1,
                        span: Default::default(),
                        ty_info: None,
                    }))
                },
            );
        }

        /// Ensure that a prefix token takes priority over a prefix test.
        #[rstest]
        fn prefix_map_precedence() {
            let mut parser = Parser::new();

            // Register the token in the map
            assert!(
                parser.register_prefix(Token::True, |_, _, _| {
                    Ok(Expression::Boolean(Boolean {
                        value: true,
                        span: Default::default(),
                        ty_info: None,
                    }))
                }),
                "register token that isn't present in the map"
            );

            // Register a test
            parser.register_prefix_test(
                |t| matches!(t, Token::True,),
                |_, _, _| {
                    Ok(Expression::Boolean(Boolean {
                        value: false,
                        span: Default::default(),
                        ty_info: None,
                    }))
                },
            );

            // Parse the expression
            let expression = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("true"),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(
                matches!(expression, Expression::Boolean(Boolean { value: true, .. })),
                "must have run parse map parser, not test function"
            );
        }

        /// Ensure that a single prefix token can be parsed.
        #[rstest]
        fn parse_single_prefix(parser: Parser) {
            let expression = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("true"),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(
                expression,
                Expression::Boolean(Boolean { value: true, .. })
            ));
        }

        /// Ensure that multiple prefix tokens can be parsed.
        #[rstest]
        fn parse_two_prefix(parser: Parser) {
            let mut lexer = Lexer::from("true false");

            let expression = parser
                .parse(&mut Compiler::default(), &mut lexer, Precedence::Lowest)
                .unwrap();

            assert!(matches!(
                expression,
                Expression::Boolean(Boolean { value: true, .. })
            ));

            let expression = parser
                .parse(&mut Compiler::default(), &mut lexer, Precedence::Lowest)
                .unwrap();

            assert!(matches!(
                expression,
                Expression::Boolean(Boolean { value: false, .. })
            ));
        }

        // Ensure that an expression can be parsed using a prefix test.
        #[rstest]
        fn parse_prefix_test(parser: Parser) {
            let expression = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("1"),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(
                expression,
                Expression::Integer(Integer { value: 1, .. })
            ));
        }
    }

    mod infix {
        use crate::hir::{Expression, Infix, InfixOperation};

        use super::*;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            // Register a simple prefix parser
            assert!(
                parser.register_prefix(Token::True, |_, _, lexer| {
                    lexer.next_token();

                    Ok(Expression::Boolean(Boolean {
                        value: true,
                        span: Default::default(),
                        ty_info: None,
                    }))
                }),
                "register prefix parser for new token"
            );

            parser.register_infix(Token::Plus, |parser, compiler, lexer, left| {
                lexer.next_token().unwrap();

                let right = parser.parse(compiler, lexer, Precedence::Sum).unwrap();

                Ok(Expression::Infix(Infix {
                    left: Box::new(left),
                    operation: InfixOperation::Plus,
                    right: Box::new(right),
                    span: Default::default(),
                    ty_info: None,
                }))
            });

            parser
        }

        /// Ensure a single infix expression can be parsed from an infix token.
        #[rstest]
        fn parse_single_infix(parser: Parser) {
            let expression = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("true + true"),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(
                expression,
                Expression::Infix(Infix {
                    operation: InfixOperation::Plus,
                    ..
                })
            ));
        }

        /// Ensire that multiple (nested) infix expressions can be parsed from infix tokens.
        #[rstest]
        fn parse_two_infix(parser: Parser) {
            let expression = dbg!(parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("true + true + true"),
                    Precedence::Lowest,
                )
                .unwrap());

            assert!(matches!(expression, Expression::Infix(_)));

            // Second infix expression
            let Expression::Infix(Infix { left, .. }) = expression else {
                unreachable!();
            };

            assert!(matches!(*left, Expression::Infix(_)));
        }

        /// Ensure that an infix operation can be parsed from an infix test.
        #[rstest]
        fn parse_infix_test() {
            let mut parser = Parser::new();
            parser.register_prefix(Token::True, |_, _, lexer| {
                lexer.next_token();
                Ok(Expression::Boolean(Boolean {
                    value: true,
                    span: Default::default(),
                    ty_info: None,
                }))
            });
            parser.register_infix_test(
                |t| matches!(t, Token::Plus),
                |parser, compiler, lexer, left| {
                    lexer.next_token();

                    let right = parser.parse(compiler, lexer, Precedence::Sum).unwrap();

                    Ok(Expression::Infix(Infix {
                        left: Box::new(left),
                        operation: InfixOperation::Plus,
                        right: Box::new(right),
                        span: Default::default(),
                        ty_info: None,
                    }))
                },
            );

            let expression = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("true + true"),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(expression, Expression::Infix(_)));
        }
    }
}
