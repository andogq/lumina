use super::*;

ast_node! {
    Array<M> {
        init: Vec<Expression<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Array<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Expression<UntypedAstMetadata>>(
            Token::LeftSquare,
            |parser, compiler, lexer| {
                // Parse opening square bracket
                let span_start = match lexer.next_spanned().unwrap() {
                    (Token::LeftSquare, span) => span.start,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::LeftSquare),
                            found: Box::new(token),
                            reason: "array literal must start with square brace".to_string(),
                        });
                    }
                };

                // Parse each of the items, deliminated by a comma
                let mut init = Vec::new();
                let mut expect_item = true;
                let span_end = loop {
                    match (lexer.peek_token().unwrap(), expect_item) {
                        (Token::Comma, false) => {
                            expect_item = true;
                            lexer.next_token();
                        }
                        (Token::RightSquare, _) => {
                            break lexer.next_spanned().unwrap().1.end;
                        }
                        (_, true) => {
                            init.push(parser.parse(compiler, lexer, Precedence::Lowest)?);
                            expect_item = false;
                        }
                        (token, _) => {
                            return Err(ParseError::ExpectedToken {
                                expected: Box::new(Token::RightSquare),
                                found: Box::new(token.clone()),
                                reason: "expected a comma or closing brace".to_string(),
                            });
                        }
                    }
                };

                Ok(Expression::Array(Array {
                    init,
                    span: span_start..span_end,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Array<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::ty::TyError> {
        // Type check each of the init items
        let init = self
            .init
            .into_iter()
            .map(|i| i.solve(compiler, state))
            .collect::<Result<Vec<_>, _>>()?;

        // Make sure all of the init items agree on the type
        let ty_info = init
            .iter()
            .map(|i| i.get_ty_info().clone())
            .collect::<Result<TyInfo, _>>()?;

        Ok(Array {
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Array {
                    inner: Box::new(ty_info.ty),
                    size: init.len() as u32,
                },
                return_ty: ty_info.return_ty,
            },
            init,
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

            Array::<UntypedAstMetadata>::register(&mut parser);

            // Use integer parser for testing
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::empty("[]", 0)]
        #[case::single("[1]", 1)]
        #[case::single_trailing("[1,]", 1)]
        #[case::double("[1, 2]", 2)]
        #[case::double_trailing("[1, 2,]", 2)]
        #[case::triple("[1, 2, 3]", 3)]
        #[case::triple_trailing("[1, 2, 3,]", 3)]
        fn flat(parser: Parser, #[case] source: &str, #[case] items: usize) {
            let array: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            assert_array_len(&array, items);
        }

        #[rstest]
        fn nested(parser: Parser) {
            let array: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from("[[1,], [1, 2,], [1, 2, 3,],]"),
                    Precedence::Lowest,
                )
                .unwrap();

            let array = assert_array_len(&array, 3);

            assert_array_len(&array.init[0], 1);
            assert_array_len(&array.init[1], 2);
            assert_array_len(&array.init[2], 3);
        }

        fn assert_array_len<M: AstMetadata>(
            expression: &Expression<M>,
            length: usize,
        ) -> &Array<M> {
            let Expression::Array(array) = expression else {
                panic!("expected to parse array");
            };

            assert_eq!(array.init.len(), length);

            array
        }
    }
}
