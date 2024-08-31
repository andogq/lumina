use crate::stage::parse::ParseError;

use super::*;

ast_node! {
    Break<M> {
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Break<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Statement<UntypedAstMetadata>>(
            Token::Break,
            |_, _, lexer| {
                let span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::Break, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Break),
                            found: Box::new(token),
                            reason: "expected break statement".to_string(),
                        });
                    }
                };

                Ok(Statement::Break(Break {
                    span,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Break<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        Ok(Break {
            ty_info: TyInfo {
                ty: Ty::Never,
                return_ty: None,
            },
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    mod parse {
        use crate::stage::parse::{Lexer, Precedence};

        use super::*;

        #[rstest]
        #[case("break")]
        fn success(#[case] source: &str) {
            let mut parser = Parser::new();
            Break::<UntypedAstMetadata>::register(&mut parser);

            let b: Statement<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(b, Statement::Break(_)));
        }
    }
}
