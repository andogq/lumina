use std::{any::TypeId, collections::HashMap};

use crate::compiler::Compiler;

use super::{Lexer, ParseError, Token};

/// Function capable of parsing an infix expression out of the provided lexer.
pub type InfixParser<T> = fn(
    parser: &Parser,
    compiler: &mut Compiler,
    lexer: &mut Lexer,
    left: T,
) -> Result<T, ParseError>;

/// Function capable of parsing a prefix expression out of the provided lexer.
pub type PrefixParser<T> =
    fn(parser: &Parser, compiler: &mut Compiler, lexer: &mut Lexer) -> Result<T, ParseError>;

/// Function to test whether a token is a match to parse.
pub type TokenTest = fn(token: &Token) -> bool;

/// Composable parser, allowing for components of the parser to be dynamically registered.
pub struct ParserRules<T> {
    /// Infix parse function to run when a given token is presented during infix parsing.
    infix: HashMap<Token, InfixParser<T>>,

    /// Dynamic tests to run on a token during infix parsing. These will be run after the infix map is checked.
    infix_tests: Vec<(TokenTest, InfixParser<T>)>,

    /// Prefix parse function to run when a given token is presented durign prefix parsing.
    prefix: HashMap<Token, PrefixParser<T>>,

    /// Dynamic tests to run on a token during prefix parsing. These will be run after the prefix map is checked.
    prefix_tests: Vec<(TokenTest, PrefixParser<T>)>,

    /// Fallback parser to use if no other parsers match.
    fallback: Option<PrefixParser<T>>,
}

impl<T> ParserRules<T> {
    /// Register a new prefix parser against a token. Will return `false` if the token has already
    /// been registered.
    pub fn register_prefix(&mut self, token: Token, parser: PrefixParser<T>) -> bool {
        // If the token has already been registered, bail
        if self.prefix.contains_key(&token) {
            return false;
        }

        self.prefix.insert(token, parser);

        true
    }

    /// Register a test for a prefix parser.
    pub fn register_prefix_test(&mut self, token_test: TokenTest, parser: PrefixParser<T>) {
        self.prefix_tests.push((token_test, parser));
    }

    /// Register a new infix parser against a token. Will return `false` if the token has already
    /// been registered.
    pub fn register_infix(&mut self, token: Token, parser: InfixParser<T>) -> bool {
        // If the token has already been registered, bail
        if self.infix.contains_key(&token) {
            return false;
        }

        self.infix.insert(token, parser);

        true
    }

    /// Register a test for an infix parser.
    pub fn register_infix_test(&mut self, token_test: TokenTest, parser: InfixParser<T>) {
        self.infix_tests.push((token_test, parser));
    }

    /// Register a fallback parser if no other parsers match the token. Will return `false` if
    /// there is already a fallback registered.
    pub fn register_fallback(&mut self, parser: PrefixParser<T>) -> bool {
        if self.fallback.is_some() {
            return false;
        }

        self.fallback = Some(parser);

        true
    }

    /// Parse an expression starting with the given precedence out of the lexer.
    pub fn parse<P: From<Token> + PartialOrd>(
        &self,
        parser: &Parser,
        compiler: &mut Compiler,
        lexer: &mut Lexer,
        precedence: P,
    ) -> Result<T, ParseError> {
        // Parse a prefix for this expression
        let mut left = self.parse_prefix(parser, compiler, lexer)?;

        while let Some(token) = lexer
            .peek_token()
            .filter(|t| precedence < P::from((*t).clone()))
        {
            let Some(infix_parser) = self.get_infix_parser(token) else {
                // Can't find an infix parser for the next token, likely finished this expression
                return Ok(left);
            };

            left = infix_parser(parser, compiler, lexer, left)?;
        }

        Ok(left)
    }

    /// Parse a prefix expression
    fn parse_prefix(
        &self,
        parser: &Parser,
        compiler: &mut Compiler,
        lexer: &mut Lexer,
    ) -> Result<T, ParseError> {
        let token = lexer.peek_token().ok_or(ParseError::UnexpectedEOF)?;

        let prefix_parser = self
            .get_prefix_parser(token)
            .or(self.fallback.as_ref())
            .ok_or_else(|| ParseError::UnexpectedToken(token.clone()))?;

        prefix_parser(parser, compiler, lexer)
    }

    /// Attempt to find a prefix parser for a given token.
    fn get_prefix_parser<'a>(&'a self, token: &'a Token) -> Option<&'a PrefixParser<T>> {
        self.get_parser(&self.prefix, &self.prefix_tests, token)
    }

    /// Attempt to find an infix parser for a given token.
    fn get_infix_parser<'a>(&'a self, token: &'a Token) -> Option<&'a InfixParser<T>> {
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

impl<T> Default for ParserRules<T> {
    fn default() -> Self {
        Self {
            infix: HashMap::new(),
            infix_tests: Vec::new(),
            prefix: HashMap::new(),
            prefix_tests: Vec::new(),
            fallback: None,
        }
    }
}

#[derive(Default)]
pub struct Parser(HashMap<TypeId, Box<dyn std::any::Any>>);

impl Parser {
    /// Create a new instance of the parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse tokens from the provided lexer in order to produce values of `T`. Will produce an
    /// error if no parser exists for `T`.
    pub fn parse<T: 'static, P: From<Token> + PartialOrd>(
        &self,
        compiler: &mut Compiler,
        lexer: &mut Lexer,
        precedence: P,
    ) -> Result<T, ParseError> {
        self.get::<T>()
            .ok_or_else(|| ParseError::NoRegisteredParsers(std::any::type_name::<T>().to_string()))?
            .parse(self, compiler, lexer, precedence)
    }

    /// Parse a sequence of items delimited by some token. Returns a [`Vec`] of the items in the
    /// order they were found, and a [`bool`] indicating whether the sequence was terminated with
    /// a delimiter.
    pub fn parse_delimited<T: 'static, P: From<Token> + PartialOrd + Default>(
        &self,
        compiler: &mut Compiler,
        lexer: &mut Lexer,
        delimiter: Token,
    ) -> (Vec<T>, bool) {
        let mut items = Vec::new();
        let mut terminated = false;

        enum ParseState {
            Item,
            Delimiter,
        }

        let mut state = ParseState::Item;

        loop {
            match state {
                ParseState::Item => {
                    // Attempt to parse an item
                    let Ok(item) = self.parse(compiler, lexer, P::default()) else {
                        break;
                    };

                    terminated = false;
                    items.push(item);

                    state = ParseState::Delimiter;
                }
                ParseState::Delimiter => {
                    if lexer
                        .peek_token()
                        .map(|token| *token != delimiter)
                        .unwrap_or(true)
                    {
                        break;
                    };

                    // Take the deliminator and mark this list as terminated
                    lexer.next_token();
                    terminated = true;

                    state = ParseState::Item;
                }
            }
        }

        (items, terminated)
    }

    /// Register a new prefix parser against a token. Will return `false` if the token has already
    /// been registered.
    pub fn register_prefix<T: 'static>(&mut self, token: Token, parser: PrefixParser<T>) -> bool {
        self.get_mut::<T>().register_prefix(token, parser)
    }

    /// Register a test for a prefix parser.
    pub fn register_prefix_test<T: 'static>(
        &mut self,
        token_test: TokenTest,
        parser: PrefixParser<T>,
    ) {
        self.get_mut::<T>().register_prefix_test(token_test, parser);
    }

    /// Register a new infix parser against a token. Will return `false` if the token has already
    /// been registered.
    pub fn register_infix<T: 'static>(&mut self, token: Token, parser: InfixParser<T>) -> bool {
        self.get_mut::<T>().register_infix(token, parser)
    }

    /// Register a test for an infix parser.
    pub fn register_infix_test<T: 'static>(
        &mut self,
        token_test: TokenTest,
        parser: InfixParser<T>,
    ) {
        self.get_mut::<T>().register_infix_test(token_test, parser);
    }

    /// Register a fallback parser if no other parsers match the token. Will return `false` if
    /// there is already a fallback registered.
    pub fn register_fallback<T: 'static>(&mut self, parser: PrefixParser<T>) -> bool {
        self.get_mut::<T>().register_fallback(parser)
    }

    /// Get a reference to the parser rules associated with some type.
    fn get<T: 'static>(&self) -> Option<&ParserRules<T>> {
        Some(
            self.0
                .get(&TypeId::of::<ParserRules<T>>())?
                .downcast_ref::<ParserRules<T>>()
                .expect("correct type stored against type ID"),
        )
    }

    /// Get a mutable reference to the parser rules associated with some type, creating it if it doesn't exist.
    fn get_mut<T: 'static>(&mut self) -> &mut ParserRules<T> {
        self.0
            .entry(TypeId::of::<ParserRules<T>>())
            .or_insert_with(|| Box::new(ParserRules::<T>::default()))
            .downcast_mut::<ParserRules<T>>()
            .expect("correct type stored against type ID")
    }
}

#[cfg(test)]
mod test {
    use crate::{
        hir::*,
        stage::parse::{Precedence, UntypedAstMetadata},
    };

    use super::*;
    use rstest::*;

    type Expression = crate::hir::Expression<UntypedAstMetadata>;

    /// Ensure parsing fails when no parsers are registered.
    #[rstest]
    fn parse_no_match() {
        let parser = Parser::new();

        let result = parser.parse::<(), _>(
            &mut Compiler::default(),
            &mut Lexer::from("true"),
            Precedence::Lowest,
        );

        assert!(matches!(result, Err(ParseError::NoRegisteredParsers(_))));
    }

    mod delimited {
        use super::*;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            parser.register_prefix(Token::True, |_, _, lexer| {
                lexer.next_token().unwrap();

                Ok(())
            });

            parser
        }

        #[rstest]
        #[case::empty("", 0, false)]
        #[case::single_unterminated("true", 1, false)]
        #[case::single_terminated("true;", 1, true)]
        #[case::double_unterminated("true; true", 2, false)]
        #[case::double_terminated("true; true;", 2, true)]
        #[case::triple_unterminated("true; true; true", 3, false)]
        #[case::triple_terminated("true; true; true;", 3, true)]
        fn success(
            parser: Parser,
            #[case] source: &str,
            #[case] expected_length: usize,
            #[case] expected_terminated: bool,
        ) {
            let (items, terminated) = parser.parse_delimited::<(), Precedence>(
                &mut Compiler::default(),
                &mut Lexer::from(source),
                Token::SemiColon,
            );

            assert_eq!(items.len(), expected_length);
            assert_eq!(terminated, expected_terminated);
        }

        #[rstest]
        #[case::delimiter(";", 0, false, &[Token::SemiColon])]
        #[case::single_trailing_token("true true", 1, false, &[Token::True])]
        #[case::double_trailing_token("true; true true", 2, false, &[Token::True])]
        #[case::single_trailing_delimiter("true;;", 1, true, &[Token::SemiColon])]
        #[case::double_trailing_delimiter("true; true;;", 2, true, &[Token::SemiColon])]
        fn handle_trailing_tokens(
            parser: Parser,
            #[case] source: &str,
            #[case] expected_length: usize,
            #[case] expected_terminated: bool,
            #[case] trailing_tokens: &[Token],
        ) {
            let mut lexer = Lexer::from(source);
            let (items, terminated) = parser.parse_delimited::<(), Precedence>(
                &mut Compiler::default(),
                &mut lexer,
                Token::SemiColon,
            );

            assert_eq!(items.len(), expected_length);
            assert_eq!(terminated, expected_terminated);

            assert_eq!(
                lexer
                    .clone()
                    .filter_map(|(t, _)| t.ok())
                    .collect::<Vec<_>>(),
                trailing_tokens
            )
        }
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
            let expression = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("true + true + true"),
                    Precedence::Lowest,
                )
                .unwrap();

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
