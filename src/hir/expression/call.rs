use super::*;

ast_node! {
    Call<M> {
        name: M::FnIdentifier,
        args: Vec<Expression<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Call<M> {
    fn register(parser: &mut Parser) {
        assert!(
            parser.register_infix(Token::LeftParen, |parser, compiler, lexer, left| {
                // Pull out a binding for the LHS
                let (binding, binding_span) = match left {
                    Expression::Ident(Ident { binding, span, .. }) => (binding, span),
                    lhs => {
                        return Err(ParseError::InvalidInfixLhs {
                            found: Box::new(lhs),
                            reason: "assign must start with ident".to_string(),
                        });
                    }
                };

                // Consume the args
                let args = std::iter::from_fn(|| {
                    match lexer.peek_token()? {
                        Token::RightParen => None,
                        Token::LeftParen | Token::Comma => {
                            // Consume the opening paren or comma
                            lexer.next_token();

                            // If the closing parenthesis is encountered, stop parsing arguments
                            if matches!(lexer.peek_token().unwrap(), Token::RightParen) {
                                return None;
                            }

                            // Parse the next argument
                            Some(parser.parse(compiler, lexer, Precedence::Lowest))
                        }
                        token => Some(Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Comma),
                            found: Box::new(token.clone()),
                            reason: "function arguments must be separated by a comma".to_string(),
                        })),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;

                // Consume the closing paren
                let end_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::RightParen, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::RightParen),
                            found: Box::new(token),
                            reason: "argument list must end with right paren".to_string(),
                        })
                    }
                };

                let span = binding_span.start..end_span.end;
                Ok(Expression::Call(Call::new(
                    binding,
                    args,
                    span,
                    Default::default(),
                )))
            })
        );
    }
}

impl SolveType for Call<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        // Determine the types of all the arguments
        let args = self
            .args
            .into_iter()
            .map(|arg| arg.solve(compiler, state))
            .collect::<Result<Vec<_>, _>>()?;

        // Compare the arguments to the function types
        let function_idx = compiler
            .functions
            .get_idx(self.name)
            .ok_or(TyError::SymbolNotFound(self.name))?;
        let signature = compiler
            .functions
            .get(function_idx)
            .expect("function must be defined")
            .get_signature();

        if args.len() != signature.arguments.len() {
            // TODO: Make new type error for when the function call has too many arguments
            panic!("too many arguments");
        }

        if !args
            .iter()
            .map(|arg| arg.get_ty_info().ty.clone())
            .zip(&signature.arguments)
            .all(|(call, signature)| call == *signature)
        {
            // TODO: New type error when a parameter is the wrong type
            panic!("parameter wong type");
        }

        Ok(Call {
            ty_info: TyInfo::try_from((
                // Function call's resulting type will be whatever the function returns
                signature.return_ty.clone(),
                // Ensure all the return types from the arguments are correct
                args.iter().map(|arg| arg.get_ty_info().return_ty.clone()),
            ))?,
            name: function_idx,
            args,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stage::parse::Lexer;
    use rstest::*;

    mod parse {
        use super::*;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            Call::<UntypedAstMetadata>::register(&mut parser);

            // Register other helpers
            Ident::<UntypedAstMetadata>::register(&mut parser);
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::no_args("myfn()", 0)]
        #[case::single_arg("myfn(a)", 1)]
        #[case::single_arg_trailing("myfn(a,)", 1)]
        #[case::double_arg("myfn(a, b)", 2)]
        #[case::double_arg_trailing("myfn(a, b,)", 2)]
        #[case::triple_arg("myfn(a, b, c)", 3)]
        #[case::triple_arg_trailing("myfn(a, b, c,)", 3)]
        fn success(parser: Parser, #[case] source: &str, #[case] arg_count: usize) {
            let mut compiler = Compiler::default();

            let call: Expression<UntypedAstMetadata> = parser
                .parse(&mut compiler, &mut Lexer::from(source), Precedence::Lowest)
                .unwrap();

            let Expression::Call(call) = call else {
                panic!("expected to parse call");
            };

            assert_eq!(call.args.len(), arg_count);
            assert_eq!("myfn", compiler.symbols.resolve(call.name).unwrap());
        }

        #[rstest]
        #[case::invalid_lhs("1(a)")]
        #[case::missing_closing("myfn(a")]
        #[case::single_comma_no_args("myfn(,)")]
        fn fail(parser: Parser, #[case] source: &str) {
            assert!(parser
                .parse::<Expression<UntypedAstMetadata>, _>(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest
                )
                .is_err());
        }
    }
}
