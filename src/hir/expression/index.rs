use crate::stage::parse::{ParseError, Precedence};

use super::*;

ast_node! {
    Index<M> {
        value: M::IdentIdentifier,
        index: Box<Expression<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Index<M> {
    fn register(parser: &mut Parser) {
        assert!(
            parser.register_infix(Token::LeftSquare, |parser, compiler, lexer, left| {
                // Ensure that left is an ident
                let (binding, binding_span) = match left {
                    Expression::Ident(Ident { binding, span, .. }) => (binding, span),
                    lhs => {
                        return Err(ParseError::InvalidInfixLhs {
                            found: Box::new(lhs),
                            reason: "index must start with an ident".to_string(),
                        });
                    }
                };

                // Parse out opening bracket
                match lexer.next_token().ok_or(ParseError::UnexpectedEOF)? {
                    Token::LeftSquare => (),
                    token => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::LeftSquare),
                            found: Box::new(token),
                            reason: "expected index operation".to_string(),
                        })
                    }
                }

                // Parse index
                let index = parser.parse(compiler, lexer, Precedence::Lowest)?;

                // Parse closing bracket
                let closing_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::RightSquare, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::RightSquare),
                            found: Box::new(token),
                            reason: "expected closing bracket for index".to_string(),
                        })
                    }
                };

                Ok(Expression::Index(Index {
                    value: binding,
                    index: Box::new(index),
                    span: binding_span.start..closing_span.end,
                    ty_info: None,
                }))
            })
        );
    }
}

impl SolveType for Index<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Ensure the inner parts are correct
        let index = self.index.solve(compiler, state)?;

        // Ensure that the index can be used as an index
        let index_ty = index.get_ty_info().ty.clone();
        if index_ty != Ty::Int {
            return Err(TyError::Mismatch(index_ty, Ty::Int));
        }

        let (value, ty) = state
            .resolve(self.value)
            .ok_or(TyError::SymbolNotFound(self.value))?;

        // Ensure the value is indexable
        let result_ty = if let Ty::Array {
            inner: inner_ty, ..
        } = ty
        {
            *inner_ty
        } else {
            return Err(TyError::Index(ty));
        };

        Ok(Index {
            value,
            span: self.span,
            ty_info: TyInfo {
                ty: result_ty,
                return_ty: index.get_ty_info().return_ty.clone(),
            },
            index: Box::new(index),
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

            Index::<UntypedAstMetadata>::register(&mut parser);

            // Register helper parsers
            Integer::<UntypedAstMetadata>::register(&mut parser);
            Ident::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::normal("a[1]", |e| matches!(e, Expression::Integer(_)))]
        #[case::nested("a[b[2]]", |e| matches!(e, Expression::Index(_)))]
        fn success(
            parser: Parser,
            #[case] source: &str,
            #[case] index_test: fn(Expression<UntypedAstMetadata>) -> bool,
        ) {
            let index = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Index(index) = index else {
                panic!("expected to parse index");
            };

            assert!(index_test(*index.index));
        }

        #[rstest]
        #[case::missing_closing_bracket("a[1")]
        #[case::missing_index("a[]")]
        #[case::non_ident_lhs("1[3]")]
        fn fail(parser: Parser, #[case] source: &str) {
            assert!(parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest
                )
                .is_err());
        }
    }
}
