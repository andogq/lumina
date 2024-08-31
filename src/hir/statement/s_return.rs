use crate::stage::parse::{ParseError, Precedence};

use super::*;

ast_node! {
    Return<M> {
        value: Expression<M>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Return<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Statement<UntypedAstMetadata>>(
            Token::Return,
            |parser, compiler, lexer| {
                // Parse out the return keyword
                let span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::Return, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Return),
                            found: Box::new(token),
                            reason: "expected return keyword".to_string(),
                        });
                    }
                };

                // Parse out the value
                let value: Expression<UntypedAstMetadata> =
                    parser.parse(compiler, lexer, Precedence::Lowest)?;

                Ok(Statement::Return(Return {
                    span: span.start..value.span().end,
                    value,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Return<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        let value = self.value.solve(compiler, state)?;

        Ok(Return {
            ty_info: TyInfo::try_from((
                Ty::Never,
                [
                    Some(value.get_ty_info().ty.clone()),
                    value.get_ty_info().return_ty.clone(),
                ],
            ))?,
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

            Return::<UntypedAstMetadata>::register(&mut parser);

            // Helper parser for testing
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        fn success(parser: Parser) {
            let r: Statement<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("return 1"),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(
                r,
                Statement::Return(Return {
                    value: Expression::Integer(Integer { value: 1, .. }),
                    ..
                })
            ));
        }

        #[rstest]
        fn fail(parser: Parser) {
            let result: Result<Statement<UntypedAstMetadata>, _> = parser.parse(
                &mut Compiler::default(),
                &mut Lexer::from("return"),
                Precedence::Lowest,
            );

            assert!(result.is_err());
        }
    }

    mod ty {
        use super::*;

        #[test]
        fn return_statement() {
            // return 0;
            let s = Statement::_return(Expression::integer(0, Span::default()), Span::default());

            let ty_info = s
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .get_ty_info()
                .clone();

            assert_eq!(ty_info.ty, Ty::Never);
            assert_eq!(ty_info.return_ty, Some(Ty::Int));
        }
    }
}
