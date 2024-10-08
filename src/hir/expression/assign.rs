use super::*;

ast_node! {
    Assign<M> {
        binding: M::IdentIdentifier,
        value: Box<Expression<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Assign<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_infix::<Expression<UntypedAstMetadata>>(
            Token::Eq,
            |parser, compiler, lexer, left| {
                let (binding, binding_span) = match left {
                    Expression::Ident(Ident { binding, span, .. }) => (binding, span),
                    lhs => {
                        return Err(ParseError::InvalidInfixLhs {
                            found: Box::new(lhs),
                            reason: "assign must start with ident".to_string(),
                        });
                    }
                };

                match lexer.next_token().unwrap() {
                    Token::Eq => (),
                    token => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Eq),
                            found: Box::new(token),
                            reason: "equals sign following binding for assign".to_string(),
                        });
                    }
                }

                let value: Expression<UntypedAstMetadata> =
                    parser.parse(compiler, lexer, Precedence::Lowest)?;

                Ok(Expression::Assign(Assign {
                    span: binding_span.start..value.span().end,
                    binding,
                    value: Box::new(value),
                    ty_info: None,
                }))
            }
        ));

        [
            (Token::AddAssign, InfixOperation::Plus),
            (Token::MinusAssign, InfixOperation::Minus),
            (Token::MulAssign, InfixOperation::Multiply),
            (Token::DivAssign, InfixOperation::Divide),
        ]
        .into_iter()
        .for_each(|(token, operation)| {
            assert!(parser.register_infix::<Expression<UntypedAstMetadata>>(
                token.clone(),
                move |parser, compiler, lexer, left| {
                    let (binding, binding_span) = match left {
                        Expression::Ident(Ident { binding, span, .. }) => (binding, span),
                        lhs => {
                            return Err(ParseError::InvalidInfixLhs {
                                found: Box::new(lhs),
                                reason: "assign must start with ident".to_string(),
                            });
                        }
                    };

                    let token_span = match lexer.next_spanned().unwrap() {
                        (t, span) if t == token => span,
                        (token, _) => {
                            return Err(ParseError::ExpectedToken {
                                expected: Box::new(Token::Eq),
                                found: Box::new(token),
                                reason: "equals sign following binding for assign".to_string(),
                            });
                        }
                    };

                    let value: Expression<UntypedAstMetadata> =
                        parser.parse(compiler, lexer, Precedence::Lowest)?;
                    let value_span = value.span();

                    Ok(Expression::Assign(Assign {
                        span: binding_span.start..value_span.end,
                        binding,
                        // Manually build an infix operation (not great, but it'll do)
                        value: Box::new(Expression::Infix(Infix {
                            left: Box::new(Expression::Ident(Ident {
                                binding,
                                span: binding_span,
                                ty_info: None,
                            })),
                            operation,
                            span: token_span.start..value_span.end,
                            right: Box::new(value),
                            ty_info: None,
                        })),
                        ty_info: None,
                    }))
                }
            ));
        });
    }
}

impl SolveType for Assign<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::ty::TyError> {
        // Work out what type the variable has to be
        let (binding, ty) = state
            .resolve(self.binding)
            .ok_or(TyError::SymbolNotFound(self.binding))?;

        let value = self.value.solve(compiler, state)?;

        let value_ty = value.get_ty_info().ty.clone();

        if value_ty != ty {
            return Err(TyError::Mismatch(ty, value_ty));
        }

        Ok(Assign {
            binding,
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: value.get_ty_info().return_ty.clone(),
            },
            value: Box::new(value),
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

            Assign::<UntypedAstMetadata>::register(&mut parser);

            // Register additional parsers for testing
            Integer::<UntypedAstMetadata>::register(&mut parser);
            Ident::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::valid_integer_rhs("myident", "1")]
        #[case::valid_ident_rhs("myident", "otherident")]
        fn success(parser: Parser, #[case] lhs: &str, #[case] rhs: &str) {
            let mut compiler = Compiler::default();

            let assign: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut compiler,
                    &mut Lexer::from(format!("{lhs} = {rhs}").as_str()),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Assign(assign) = assign else {
                panic!("expected to parse assignment")
            };

            assert_eq!(lhs, compiler.symbols.resolve(assign.binding).unwrap());
        }

        #[rstest]
        #[case::add_assign("+=", InfixOperation::Plus)]
        #[case::minus_assign("-=", InfixOperation::Minus)]
        #[case::mul_assign("*=", InfixOperation::Multiply)]
        #[case::div_assign("/=", InfixOperation::Divide)]
        fn op_assign(parser: Parser, #[case] op: &str, #[case] expected: InfixOperation) {
            let mut compiler = Compiler::default();

            let assign: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut compiler,
                    &mut Lexer::from(format!("ident {op} 1").as_str()),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Assign(Assign { value, .. }) = assign else {
                panic!("expected to parse assignment")
            };
            let Expression::Infix(Infix { operation, .. }) = *value else {
                panic!("expected assignment to contain infix operation");
            };

            assert_eq!(operation, expected);
        }

        #[rstest]
        fn invalid(parser: Parser) {
            let result: Result<Expression<UntypedAstMetadata>, _> = parser.parse(
                &mut Compiler::default(),
                &mut Lexer::from("1 = otherident"),
                Precedence::Lowest,
            );

            assert!(matches!(result, Err(ParseError::InvalidInfixLhs { .. })));
        }
    }
}
