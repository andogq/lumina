use crate::stage::parse::ParseError;

use super::*;

ast_node! {
    Continue<TyInfo> {
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Continue<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Statement<UntypedAstMetadata>>(
            Token::Continue,
            |_, _, lexer| {
                let continue_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::Continue, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Continue),
                            found: Box::new(token),
                            reason: "expected continue statement".to_string(),
                        });
                    }
                };

                // Parse out the semicolon
                let semicolon_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::SemiColon, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::SemiColon),
                            found: Box::new(token),
                            reason: "expected continue statement to finish with semicolon"
                                .to_string(),
                        });
                    }
                };

                Ok(Statement::Continue(Continue {
                    span: continue_span.start..semicolon_span.end,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Continue<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        Ok(Continue {
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

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();
            Continue::<UntypedAstMetadata>::register(&mut parser);
            parser
        }

        #[rstest]
        #[case("continue;")]
        fn success(#[case] source: &str) {
            let mut parser = Parser::new();
            Continue::<UntypedAstMetadata>::register(&mut parser);

            let c: Statement<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(c, Statement::Continue(_)));
        }

        #[rstest]
        #[case::missing_semicolon("continue")]
        fn fail(parser: Parser, #[case] source: &str) {
            assert!(parser
                .parse::<Statement<UntypedAstMetadata>, _>(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .is_err());
        }
    }
}
