use super::*;

ast_node! {
    Block<M> {
        statements: Vec<Statement<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Block<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_prefix::<Expression<UntypedAstMetadata>>(
            Token::LeftBrace,
            |parser, compiler, lexer| {
                // Parse opening bracket
                let start_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::LeftBrace, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::LeftBrace),
                            found: Box::new(token),
                            reason: "block must start with opening brace".to_string(),
                        });
                    }
                };

                // Parse statements
                let mut statements = Vec::new();
                while lexer
                    .peek_token()
                    .map(|t| !matches!(t, Token::RightBrace))
                    .unwrap_or(false)
                {
                    statements.push(parser.parse(compiler, lexer, Precedence::Lowest)?);
                }

                // Parse ending bracket
                let end_span = match lexer.next_spanned().ok_or(ParseError::UnexpectedEOF)? {
                    (Token::RightBrace, span) => span,
                    (token, _) => {
                        return Err(ParseError::ExpectedToken {
                            expected: Box::new(Token::RightBrace),
                            found: Box::new(token),
                            reason: "block must end with closing brace".to_string(),
                        });
                    }
                };

                Ok(Expression::Block(Block {
                    statements,
                    span: start_span.start..end_span.end,
                    ty_info: None,
                }))
            }
        ));
    }
}

impl SolveType for Block<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::ty::TyError> {
        // Enter a new scope
        let block_scope = state.enter();

        let statements = self
            .statements
            .into_iter()
            .map(|statement| statement.solve(compiler, state))
            .collect::<Result<Vec<_>, _>>()?;

        let ty_info = TyInfo::try_from((
            // Type of this block will be the implicit return of the last block
            statements
                .last()
                // The block can only inherit the type of an expression statement
                // .filter(|s| {
                //     matches!(
                //         s,
                //         Statement::Expression(ExpressionStatement {
                //             implicit_return: true,
                //             ..
                //         })
                //     )
                // })
                .map(|s| s.get_ty_info().ty.clone())
                .unwrap_or(Ty::Unit),
            statements
                .iter()
                .map(|statement| statement.get_ty_info().return_ty.clone()),
        ))?;

        // Leave a scope
        assert_eq!(
            block_scope,
            state.leave(),
            "ensure the scope that is left was the same that was entered"
        );

        Ok(Block {
            span: self.span,
            statements,
            ty_info,
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

            Block::<UntypedAstMetadata>::register(&mut parser);

            // Helpers
            ExpressionStatement::<UntypedAstMetadata>::register(&mut parser);
            Boolean::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::empty("{ }", 0)]
        #[case::single_terminated("{ true; }", 1)]
        #[case::single_unterminated("{ true }", 1)]
        #[case::double_terminated("{ true; true; }", 2)]
        #[case::double_unterminated("{ true; true }", 2)]
        #[case::triple_terminated("{ true; true; true; }", 3)]
        #[case::triple_unterminated("{ true; true; true }", 3)]
        fn success(parser: Parser, #[case] source: &str, #[case] count: usize) {
            let b: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Block(b) = b else {
                panic!("expected to parse block");
            };

            assert_eq!(b.statements.len(), count);
        }
    }
}
