use crate::stage::parse::ParseError;

use super::*;

ast_node! {
    Integer<M> {
        value: i64,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Integer<M> {
    fn register(parser: &mut Parser) {
        parser.register_prefix_test(
            |token| matches!(token, Token::Integer(_)),
            |_, _, lexer| {
                let (value, span) = match lexer.next_spanned().unwrap() {
                    (Token::Integer(value), span) => (value, span),
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Integer(0)),
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
        )
    }
}

impl SolveType for Integer<UntypedAstMetadata> {
    type State = Scope;

    fn solve(self, _compiler: &mut Compiler, _scope: &mut Scope) -> Result<Self::Typed, TyError> {
        Ok(Integer {
            value: self.value,
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Int,
                return_ty: None,
            },
        })
    }
}

#[cfg(test)]
mod test_integer {
    use super::*;

    use rstest::*;

    mod parse {
        use crate::stage::parse::{Lexer, Precedence};

        use super::*;

        #[rstest]
        #[case::single_digit(1)]
        #[case::multi_digit(123)]
        fn success(#[case] value: i64) {
            let mut parser = Parser::new();

            Integer::<UntypedAstMetadata>::register(&mut parser);

            let integer = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(value.to_string().as_str()),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Integer(integer) = integer else {
                panic!("expected integer to be returned");
            };

            assert_eq!(integer.value, value);
        }
    }

    mod ty {
        use super::*;
        #[test]
        fn integer_infer() {
            assert_eq!(
                Integer::new(0, Span::default(), Default::default())
                    .solve(&mut Compiler::default(), &mut Scope::new())
                    .unwrap()
                    .ty_info
                    .ty,
                Ty::Int
            );
        }

        #[test]
        fn integer_return() {
            assert_eq!(
                Integer::new(0, Span::default(), Default::default())
                    .solve(&mut Compiler::default(), &mut Scope::new())
                    .unwrap()
                    .ty_info
                    .return_ty,
                None,
            );
        }
    }
}
