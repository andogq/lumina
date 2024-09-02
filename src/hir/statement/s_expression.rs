use crate::stage::parse::Precedence;

use super::*;

ast_node! {
    ExpressionStatement<M> {
        expression: Expression<M>,
        terminated: bool,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for ExpressionStatement<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_fallback::<Statement<UntypedAstMetadata>>(
            |parser, compiler, lexer| {
                let expression: Expression<UntypedAstMetadata> =
                    parser.parse(compiler, lexer, Precedence::Lowest)?;

                let terminating_span = lexer
                    .next_if(|(token, _)| matches!(token, Ok(Token::SemiColon)))
                    .map(|(_, span)| span);

                let expression_span = expression.span().clone();
                let span = expression_span.start
                    ..terminating_span
                        .as_ref()
                        .map(|span| span.end)
                        .unwrap_or(expression_span.end);

                Ok(Statement::ExpressionStatement(ExpressionStatement {
                    span,
                    terminated: terminating_span.is_some(),
                    expression,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for ExpressionStatement<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        let expression = self.expression.solve(compiler, state)?;

        // Expression statement has same type as the underlying expression
        let ty_info = expression.get_ty_info().clone();

        Ok(ExpressionStatement {
            ty_info: TyInfo {
                ty: if self.terminated {
                    Ty::Unit
                } else {
                    ty_info.ty
                },
                return_ty: ty_info.return_ty,
            },
            expression,
            terminated: self.terminated,
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

            ExpressionStatement::<UntypedAstMetadata>::register(&mut parser);

            // Register some helpers
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::integer_unterminated("1", false, |e| matches!(e, Expression::Integer(_)))]
        #[case::integer_unterminated("1;", true, |e| matches!(e, Expression::Integer(_)))]
        fn success(
            parser: Parser,
            #[case] source: &str,
            #[case] expect_terminated: bool,
            #[case] tester: fn(Expression<UntypedAstMetadata>) -> bool,
        ) {
            let s: Statement<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Statement::ExpressionStatement(ExpressionStatement {
                expression,
                terminated,
                ..
            }) = s
            else {
                panic!("expected expression statement");
            };

            assert_eq!(terminated, expect_terminated);

            assert!(tester(expression));
        }
    }

    mod ty {
        use super::*;

        #[rstest]
        #[case(Ty::Int, false)]
        #[case(Ty::Unit, true)]
        fn infer(#[case] ty: Ty, #[case] terminated: bool) {
            let s = Statement::expression(
                Expression::integer(0, Span::default()),
                terminated,
                Span::default(),
            );

            let ty_info = s
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .get_ty_info()
                .clone();

            assert_eq!(ty_info.ty, ty);
            assert_eq!(ty_info.return_ty, None);
        }
    }
}
