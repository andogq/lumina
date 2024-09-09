use super::*;

ast_node! {
    Let<M> {
        binding: M::IdentIdentifier,
        value: Expression<M>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Let<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Statement<UntypedAstMetadata>>(
            Token::Let,
            |parser, compiler, lexer| {
                // Parse out `let`
                let start_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::Let, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Let),
                            found: Box::new(token),
                            reason: "expected let token".to_string(),
                        });
                    }
                };

                // Parse out binding
                let binding = match lexer.next_token().ok_or(ParseError::UnexpectedEOF)? {
                    Token::Ident(ident) => compiler.symbols.get_or_intern(ident),
                    token => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Ident(String::new())),
                            found: Box::new(token),
                            reason: "expected identifier for let statement".to_string(),
                        });
                    }
                };

                // Parse out equals sign
                match lexer.next_token().ok_or(ParseError::UnexpectedEOF)? {
                    Token::Eq => (),
                    token => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Eq),
                            found: Box::new(token),
                            reason: "expected assignment following binding".to_string(),
                        });
                    }
                }

                // Parse out value
                let value: Expression<UntypedAstMetadata> =
                    parser.parse(compiler, lexer, Precedence::Lowest)?;

                // Parse out the semicolon
                let semicolon_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::SemiColon, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::SemiColon),
                            found: Box::new(token),
                            reason: "expected let statement to finish with semicolon".to_string(),
                        });
                    }
                };

                Ok(Statement::Let(Let {
                    binding,
                    span: start_span.start..semicolon_span.end,
                    value,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Let<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        // Work out what the type of the value is
        let value = self.value.solve(compiler, state)?;

        // Make sure the value type matches what the statement was annotated with
        if let Some(ty) = self.ty_info {
            let value_ty = value.get_ty_info();
            if !ty.check(&value_ty.ty) {
                return Err(TyError::Mismatch(ty, value_ty.ty.clone()));
            }
        }

        // Record the type
        let binding = state.register(self.binding, value.get_ty_info().ty.clone());

        Ok(Let {
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: value.get_ty_info().return_ty.clone(),
            },
            binding,
            value,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    mod parse {
        use crate::stage::parse::Lexer;

        use super::*;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            Let::<UntypedAstMetadata>::register(&mut parser);

            // Extra helpers
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::regular("let a = 1;")]
        fn success(parser: Parser, #[case] source: &str) {
            let mut compiler = Compiler::default();

            let s: Statement<UntypedAstMetadata> = parser
                .parse(&mut compiler, &mut Lexer::from(source), Precedence::Lowest)
                .unwrap();

            let Statement::Let(Let { binding, value, .. }) = s else {
                panic!("expected to parse let statement");
            };

            assert_eq!(compiler.symbols.resolve(binding).unwrap(), "a");

            assert!(matches!(
                value,
                Expression::Integer(Integer { value: 1, .. }),
            ));
        }

        #[rstest]
        #[case::missing_binding("let = 1;")]
        #[case::missing_equals("let a 1;")]
        #[case::missing_value("let a =;")]
        #[case::missing_semicolon("let a = 1")]
        fn fail(parser: Parser, #[case] source: &str) {
            assert!(parser
                .parse::<Statement<UntypedAstMetadata>, _>(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .is_err())
        }
    }

    mod ty {
        use string_interner::Symbol;

        use super::*;

        #[test]
        fn let_statement() {
            // let a = 0;
            let s = Statement::_let(
                Symbol::try_from_usize(0).unwrap(),
                Expression::integer(0, Span::default()),
                Span::default(),
            );

            let mut scope = Scope::new();

            let ty_info = s
                .solve(&mut Compiler::default(), &mut scope)
                .unwrap()
                .get_ty_info()
                .clone();

            assert_eq!(ty_info.ty, Ty::Unit);
            assert_eq!(ty_info.return_ty, None);
            assert_eq!(
                scope.resolve(Symbol::try_from_usize(0).unwrap()).unwrap().1,
                Ty::Int
            );
        }
    }
}
