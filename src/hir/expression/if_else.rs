use crate::stage::parse::{ParseError, Precedence};

use super::*;

ast_node! {
    If<M> {
        condition: Box<Expression<M>>,
        success: Block<M>,
        otherwise: Option<Block<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for If<M> {
    fn register(parser: &mut Parser) {
        assert!(
            parser.register_prefix(Token::If, |parser, compiler, lexer| {
                let span_start = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::If, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::If),
                            found: Box::new(token),
                            reason: "expected if statement".to_string(),
                        });
                    }
                };

                // Parse condition
                let condition = parser.parse(compiler, lexer, Precedence::Lowest)?;

                // Parse out success block
                let Expression::<UntypedAstMetadata>::Block(success) =
                    parser.parse(compiler, lexer, Precedence::Lowest)?
                else {
                    return Err(ParseError::ExpectedBlock);
                };

                let otherwise = matches!(lexer.peek_token(), Some(Token::Else))
                    .then(|| {
                        lexer.next_token().unwrap();

                        let Expression::<UntypedAstMetadata>::Block(otherwise) =
                            parser.parse(compiler, lexer, Precedence::Lowest)?
                        else {
                            return Err(ParseError::ExpectedBlock);
                        };

                        Ok(otherwise)
                    })
                    .transpose()?;

                let span_end = otherwise
                    .as_ref()
                    .map(|otherwise| otherwise.span.clone())
                    .unwrap_or(success.span.clone());

                Ok(Expression::If(If {
                    condition: Box::new(condition),
                    success,
                    otherwise,
                    span: span_start.start..span_end.end,
                    ty_info: None,
                }))
            })
        );
    }
}

impl SolveType for If<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Make sure the condition is correctly typed
        let condition = self.condition.solve(compiler, state)?;
        let condition_ty = condition.get_ty_info();
        if !condition_ty.ty.check(&Ty::Boolean) {
            return Err(TyError::Mismatch(Ty::Boolean, condition_ty.ty.clone()));
        }

        let success = self.success.solve(compiler, state)?;
        let otherwise = self
            .otherwise
            .map(|otherwise| otherwise.solve(compiler, state))
            .transpose()?;

        let ty_info = TyInfo::try_from((
            // Branches must have the same type
            [
                success.ty_info.ty.clone(),
                otherwise
                    .as_ref()
                    .map(|otherwise| otherwise.ty_info.ty.clone())
                    .unwrap_or(Ty::Unit),
            ],
            // Any potential place for a return statement must be accounted for
            [
                condition_ty.return_ty.clone(),
                success.ty_info.return_ty.clone(),
                otherwise
                    .as_ref()
                    .and_then(|otherwise| otherwise.ty_info.return_ty.clone()),
            ],
        ))?;

        Ok(If {
            ty_info,
            condition: Box::new(condition),
            success,
            otherwise,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    mod parse {
        use super::*;
        use crate::stage::parse::Lexer;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            If::<UntypedAstMetadata>::register(&mut parser);

            // Register helper parser for tests
            ExpressionStatement::<UntypedAstMetadata>::register(&mut parser);
            Block::<UntypedAstMetadata>::register(&mut parser);
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        fn is_integer(expression: Expression<UntypedAstMetadata>) -> bool {
            matches!(expression, Expression::Integer(_))
        }

        fn is_integer_block(block: Block<UntypedAstMetadata>) -> bool {
            matches!(
                block.statements[0],
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::Integer(_),
                    ..
                })
            )
        }

        #[rstest]
        #[case::normal_if("if 123 { 1 }", is_integer, is_integer_block, None)]
        #[case::normal_if_else(
            "if 123 { 1 } else { 1 }",
            is_integer,
            is_integer_block,
            // Weird type has to be provided bc it's infered too tightly
            Some::<fn (Block<UntypedAstMetadata>) -> bool>(is_integer_block)
        )]
        #[case::nested_if_if("if 123 { if 456 { 1 } }", is_integer, |block: Block<UntypedAstMetadata>| {
            matches!(
                block.statements[0],
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::If(If {
                        otherwise: None,
                        ..
                    }),
                    ..
                })
            )
        }, None)]
        #[case::nested_if_if_else("if 123 { if 456 { 1 } else { 2 } }", is_integer, |block: Block<UntypedAstMetadata>| {
            matches!(
                block.statements[0],
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::If(If {
                        otherwise: Some(_),
                        ..
                    }),
                    ..
                })
            )
        }, None)]
        #[case::nested_if_if_else_else("if 123 { if 456 { 1 } else { 2 } } else { 3 }", is_integer, |block: Block<UntypedAstMetadata>| {
            matches!(
                block.statements[0],
                Statement::ExpressionStatement(ExpressionStatement {
                    expression: Expression::If(If {
                        otherwise: Some(_),
                        ..
                    }),
                    ..
                })
            )
        },
        Some::<fn (Block<UntypedAstMetadata>) -> bool>(is_integer_block))]
        fn success(
            parser: Parser,
            #[case] source: &str,
            #[case] condition_test: fn(Expression<UntypedAstMetadata>) -> bool,
            #[case] success_test: fn(Block<UntypedAstMetadata>) -> bool,
            #[case] otherwise_test: Option<fn(Block<UntypedAstMetadata>) -> bool>,
        ) {
            let eif = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::If(eif) = eif else {
                panic!("expected to parse if condition");
            };

            assert!(condition_test(*eif.condition));
            assert!(success_test(eif.success));

            match (eif.otherwise, otherwise_test) {
                (Some(otherwise), Some(otherwise_test)) => assert!(otherwise_test(otherwise)),
                (Some(_), None) => panic!("otherwise branch encountered with no test for it"),
                (None, Some(_)) => panic!("otherwise branch expected, but could not be parsed"),
                (None, None) => {}
            };
        }

        #[rstest]
        #[case::missing_block("if 1")]
        #[case::expression_after_condition("if 1 1")]
        #[case::if_else_immediate("if else { 1 }")]
        fn fail(parser: Parser, #[case] source: &str) {
            assert!(parser
                .parse::<Expression<UntypedAstMetadata>, _>(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest
                )
                .is_err())
        }
    }
}
