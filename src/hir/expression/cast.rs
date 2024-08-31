use crate::stage::parse::{parse_ty, ParseError};

use super::*;

ast_node! {
    Cast<M> {
        value: Box<Expression<M>>,
        target_ty: Ty,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Cast<M> {
    fn register(parser: &mut Parser) {
        assert!(parser.register_infix(Token::As, |_, _, lexer, left| {
            let as_span = match lexer.next_spanned().unwrap() {
                (Token::As, span) => span,
                (token, _) => {
                    return Err(ParseError::ExpectedToken {
                        expected: Box::new(Token::As),
                        found: Box::new(token),
                        reason: "expected to find cast expression".to_string(),
                    });
                }
            };

            // Parse out type from right hand side
            let (target_ty, target_ty_span) = parse_ty(lexer)?;

            Ok(Expression::Cast(Cast {
                value: Box::new(left),
                target_ty,
                span: as_span.start..target_ty_span.end,
                ty_info: None,
            }))
        }));
    }
}

impl SolveType for Cast<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        let value = self.value.solve(compiler, state)?;

        // Make sure that the value can be cast to the desired type
        match (value.get_ty_info().ty.clone(), self.target_ty.clone()) {
            // Unsigned integer can become signed
            (Ty::Uint, Ty::Int) => (),
            // Signed integer can loose sign
            (Ty::Int, Ty::Uint) => (),
            (lhs, rhs) => return Err(TyError::Cast(lhs, rhs)),
        }

        Ok(Cast {
            target_ty: self.target_ty.clone(),
            span: self.span,
            ty_info: TyInfo {
                ty: self.target_ty.clone(),
                return_ty: value.get_ty_info().return_ty.clone(),
            },
            value: Box::new(value),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stage::parse::Lexer;
    use crate::stage::parse::Precedence;
    use rstest::*;

    #[fixture]
    fn parser() -> Parser {
        let mut parser = Parser::new();

        Cast::<UntypedAstMetadata>::register(&mut parser);

        // Helper parsers
        Integer::<UntypedAstMetadata>::register(&mut parser);

        parser
    }

    #[rstest]
    #[case::single_cast("1 as int", |e| matches!(e, Expression::Integer(_)))]
    #[case::double_cast("1 as int as uint", |e| matches!(e, Expression::Cast(_)))]
    fn success(
        parser: Parser,
        #[case] source: &str,
        #[case] test: fn(Expression<UntypedAstMetadata>) -> bool,
    ) {
        let cast = parser
            .parse(
                &mut Compiler::default(),
                &mut Lexer::from(source),
                Precedence::Lowest,
            )
            .unwrap();

        let Expression::Cast(cast) = dbg!(cast) else {
            panic!("expected to parse cast");
        };

        assert!(test(*cast.value));
    }

    #[rstest]
    #[case::missing_type("1 as")]
    #[case::repeated_as("1 as as")]
    fn fail(parser: Parser, #[case] source: &str) {
        let result = parser.parse(
            &mut Compiler::default(),
            &mut Lexer::from(source),
            Precedence::Lowest,
        );

        assert!(result.is_err());
    }
}
