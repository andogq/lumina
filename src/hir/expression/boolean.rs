use crate::stage::parse::{Lexer, ParseError};

use super::*;

ast_node! {
    Boolean<M> {
        value: bool,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Boolean<M> {
    fn register(parser: &mut Parser) {
        fn parse(lexer: &mut Lexer) -> Result<Expression<UntypedAstMetadata>, ParseError> {
            let (token, span) = lexer.next_spanned().unwrap();

            let value = match token {
                Token::True => true,
                Token::False => false,
                token => {
                    return Err(ParseError::ExpectedToken {
                        expected: Box::new(Token::True),
                        found: Box::new(token),
                        reason: "expected boolean".to_string(),
                    });
                }
            };

            Ok(Expression::Boolean(Boolean {
                value,
                span,
                ty_info: None,
            }))
        }

        assert!(
            parser.register_prefix(Token::True, |_, _, lexer| parse(lexer)),
            "successfully register parser for `true` token"
        );
        assert!(
            parser.register_prefix(Token::False, |_, _, lexer| parse(lexer)),
            "successfully register parser for `false` token"
        );
    }
}

impl SolveType for Boolean<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut crate::compiler::Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        Ok(Boolean {
            value: self.value,
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Boolean,
                return_ty: None,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::*;

    mod parse {
        use super::*;
        use crate::stage::parse::Precedence;

        #[rstest]
        #[case::t_true("true", true)]
        #[case::t_false("false", false)]
        fn success(#[case] source: &str, #[case] value: bool) {
            let mut parser = Parser::new();

            Boolean::<UntypedAstMetadata>::register(&mut parser);

            let boolean = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Boolean(boolean) = boolean else {
                panic!("expected boolean to be returned");
            };

            assert_eq!(boolean.value, value);
        }
    }

    mod ty {
        use super::*;

        #[test]
        fn boolean_infer() {
            assert_eq!(
                Boolean::new(false, Span::default(), Default::default())
                    .solve(&mut Compiler::default(), &mut Scope::new())
                    .unwrap()
                    .ty_info
                    .ty,
                Ty::Boolean
            );
        }

        #[test]
        fn boolean_return() {
            assert_eq!(
                Boolean::new(false, Span::default(), Default::default())
                    .solve(&mut Compiler::default(), &mut Scope::new())
                    .unwrap()
                    .ty_info
                    .return_ty,
                None,
            );
        }
    }
}
