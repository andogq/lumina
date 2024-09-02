use crate::stage::parse::{ParseError, Precedence};

use super::*;

ast_node! {
    Loop<M> {
        body: Block<M>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Loop<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Expression<UntypedAstMetadata>>(
            Token::Loop,
            |parser, compiler, lexer| {
                let start_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::Loop, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::Loop),
                            found: Box::new(token),
                            reason: "expected loop statement".to_string(),
                        });
                    }
                };

                // Parse out the block
                let Expression::Block(body) = parser.parse::<Expression<UntypedAstMetadata>, _>(
                    compiler,
                    lexer,
                    Precedence::Lowest,
                )?
                else {
                    return Err(ParseError::ExpectedBlock);
                };

                Ok(Expression::Loop(Loop {
                    span: start_span.start..body.span.end,
                    body,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Loop<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Type check the body
        let body = self.body.solve(compiler, state)?;

        // TODO: Temporary whilst can't break expression
        match body.ty_info.ty {
            Ty::Unit | Ty::Never => (),
            ty => {
                return Err(TyError::Mismatch(Ty::Unit, ty));
            }
        };

        Ok(Loop {
            ty_info: body.ty_info.clone(),
            body,
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

            Loop::<UntypedAstMetadata>::register(&mut parser);

            // Helpers
            ExpressionStatement::<UntypedAstMetadata>::register(&mut parser);
            Block::<UntypedAstMetadata>::register(&mut parser);
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::empty("loop { }")]
        #[case::terminated_block("loop { 1; }")]
        #[case::unterminated_block("loop { 1 }")]
        #[case::nested_loop("loop { loop { 1; } }")]
        fn success(parser: Parser, #[case] source: &str) {
            let l: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            assert!(matches!(l, Expression::Loop(_)));
        }

        #[rstest]
        #[case::missing_block("loop")]
        #[case::expression_without_block("loop 1")]
        fn fail(parser: Parser, #[case] source: &str) {
            assert!(parser
                .parse::<Expression<UntypedAstMetadata>, _>(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest
                )
                .is_err());
        }
    }
}
