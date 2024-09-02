use crate::stage::parse::{Lexer, ParseError, Precedence};

use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InfixOperation {
    Minus,
    Plus,
    Multiply,
    Divide,
    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    And,
    Or,
}

impl InfixOperation {
    pub fn plus() -> Self {
        Self::Plus
    }

    pub fn minus() -> Self {
        Self::Minus
    }

    /// Determine the resulting type if this operator is applied to the provided parameters.
    pub fn result_ty(&self, left: &Ty, right: &Ty) -> Result<Ty, TyError> {
        use InfixOperation::*;

        match (self, left, right) {
            (Plus | Minus | Multiply | Divide, Ty::Int, Ty::Int) => Ok(Ty::Int),
            (Eq | NotEq | Greater | Less | GreaterEq | LessEq, left, right)
                if left.check(right) =>
            {
                Ok(Ty::Boolean)
            }
            (And | Or, Ty::Boolean, Ty::Boolean) => Ok(Ty::Boolean),
            (_, left, right) => Err(TyError::Mismatch(left.clone(), right.clone())),
        }
    }
}

impl TryFrom<Token> for InfixOperation {
    type Error = Token;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Plus => Ok(InfixOperation::Plus),
            Token::Minus => Ok(InfixOperation::Minus),
            Token::Asterix => Ok(InfixOperation::Multiply),
            Token::ForwardSlash => Ok(InfixOperation::Divide),
            Token::DoubleEq => Ok(InfixOperation::Eq),
            Token::NotEq => Ok(InfixOperation::NotEq),
            Token::LeftAngle => Ok(InfixOperation::Less),
            Token::RightAngle => Ok(InfixOperation::Greater),
            Token::LeftAngleEq => Ok(InfixOperation::LessEq),
            Token::RightAngleEq => Ok(InfixOperation::GreaterEq),
            Token::And => Ok(InfixOperation::And),
            Token::Or => Ok(InfixOperation::Or),
            token => Err(token),
        }
    }
}

ast_node! {
    Infix<M> {
        left: Box<Expression<M>>,
        operation: InfixOperation,
        right: Box<Expression<M>>,
        span,
        ty_info,
    }
}

impl<M: AstMetadata> Parsable for Infix<M> {
    fn register(parser: &mut Parser) {
        fn parse(
            parser: &Parser,
            compiler: &mut Compiler,
            lexer: &mut Lexer,
            left: Expression<UntypedAstMetadata>,
        ) -> Result<Expression<UntypedAstMetadata>, ParseError> {
            // Work out what the operation is
            let operation =
                InfixOperation::try_from(lexer.next_token().ok_or(ParseError::UnexpectedEOF)?)
                    .map_err(|token| ParseError::ExpectedToken {
                        // WARN: Should be any of the infix tokens
                        expected: Box::new(Token::Plus),
                        found: Box::new(token),
                        reason: "expected to parse valid infix operation".to_string(),
                    })?;

            // Parse the right side with associated precedence
            let right: Expression<UntypedAstMetadata> =
                parser.parse(compiler, lexer, Precedence::from(operation))?;

            Ok(Expression::Infix(Infix {
                span: left.span().start..right.span().end,
                left: Box::new(left),
                operation,
                right: Box::new(right),
                ty_info: None,
            }))
        }

        // Register the same parser for all of the possible infix operations
        [
            Token::Plus,
            Token::Minus,
            Token::Asterix,
            Token::ForwardSlash,
            Token::DoubleEq,
            Token::NotEq,
            Token::LeftAngle,
            Token::RightAngle,
            Token::LeftAngleEq,
            Token::RightAngleEq,
            Token::And,
            Token::Or,
        ]
        .into_iter()
        .for_each(|token| {
            assert!(parser.register_infix(token, parse));
        });
    }
}

impl SolveType for Infix<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        let left = self.left.solve(compiler, state)?;
        let right = self.right.solve(compiler, state)?;

        let left_ty_info = left.get_ty_info();
        let right_ty_info = right.get_ty_info();

        let ty_info = TyInfo::try_from((
            // Resulting type is whatever the infix operator results in
            self.operation
                .result_ty(&left_ty_info.ty, &right_ty_info.ty)?,
            [
                left_ty_info.return_ty.clone(),
                right_ty_info.return_ty.clone(),
            ],
        ))?;

        Ok(Infix {
            left: Box::new(left),
            right: Box::new(right),
            operation: self.operation,
            span: self.span,
            ty_info,
        })
    }
}

#[cfg(test)]
mod test_infix {
    use super::*;
    use rstest::*;

    mod parse {
        use super::*;

        #[fixture]
        fn parser() -> Parser {
            let mut parser = Parser::new();

            Infix::<UntypedAstMetadata>::register(&mut parser);

            // Helpers
            Integer::<UntypedAstMetadata>::register(&mut parser);

            parser
        }

        #[rstest]
        #[case::subtraction("1 - 1", InfixOperation::Minus)]
        #[case::addition("1 + 1", InfixOperation::Plus)]
        #[case::multiplication("1 * 1", InfixOperation::Multiply)]
        #[case::division("1 / 1", InfixOperation::Divide)]
        #[case::equal("1 == 1", InfixOperation::Eq)]
        #[case::not_equal("1 != 1", InfixOperation::NotEq)]
        #[case::greater("1 > 1", InfixOperation::Greater)]
        #[case::less("1 < 1", InfixOperation::Less)]
        #[case::greater_equal("1 >= 1", InfixOperation::GreaterEq)]
        #[case::less_equal("1 <= 1", InfixOperation::LessEq)]
        #[case::and("1 && 1", InfixOperation::And)]
        #[case::or("1 || 1", InfixOperation::Or)]
        fn success(parser: Parser, #[case] source: &str, #[case] operation: InfixOperation) {
            let e: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Infix(e) = e else {
                panic!("expected to parse infix expression");
            };

            assert_eq!(e.operation, operation);
        }

        #[rstest]
        #[case("1 + 1 + 1", InfixOperation::Plus, |left| matches!(left, Expression::Infix(Infix { operation: InfixOperation::Plus, .. })), |right| matches!(right, Expression::Integer(_)))]
        #[case("1 + 1 * 1", InfixOperation::Plus, |left| matches!(left, Expression::Integer(_)), |right| matches!(right, Expression::Infix(Infix { operation: InfixOperation::Multiply, ..})))]
        #[case("1 + 1 * 1 == 1", InfixOperation::Eq, |left| matches!(left, Expression::Infix(Infix { operation: InfixOperation::Plus, .. })), |right| matches!(right, Expression::Integer(_)))]
        #[case("1 + 1 * 1 == 1 + 1", InfixOperation::Eq,|left| matches!(left, Expression::Infix(Infix { operation: InfixOperation::Plus, .. })), |right| matches!(right, Expression::Infix(Infix { operation: InfixOperation::Plus, .. })))]
        fn precedence(
            parser: Parser,
            #[case] source: &str,
            #[case] op: InfixOperation,
            #[case] test_left: fn(Expression<UntypedAstMetadata>) -> bool,
            #[case] test_right: fn(Expression<UntypedAstMetadata>) -> bool,
        ) {
            let e: Expression<UntypedAstMetadata> = parser
                .parse(
                    &mut Compiler::default(),
                    &mut Lexer::from(source),
                    Precedence::Lowest,
                )
                .unwrap();

            let Expression::Infix(e) = e else {
                panic!("expected to parse infix expression");
            };

            assert_eq!(op, e.operation);

            assert!(test_left(*e.left));
            assert!(test_right(*e.right));
        }
    }

    mod ty {
        use super::*;

        #[test]
        fn infix_same() {
            // 0 + 0
            let infix = Infix::new(
                Box::new(Expression::integer(0, Span::default())),
                InfixOperation::plus(),
                Box::new(Expression::integer(0, Span::default())),
                Span::default(),
                Default::default(),
            );

            let ty_info = infix
                .solve(&mut Compiler::default(), &mut Scope::new())
                .unwrap()
                .ty_info;
            assert_eq!(ty_info.ty, Ty::Int);
            assert_eq!(ty_info.return_ty, None);
        }
        #[test]
        fn infix_different() {
            // 0 + false
            let infix = Infix::new(
                Box::new(Expression::integer(0, Span::default())),
                InfixOperation::plus(),
                Box::new(Expression::boolean(false, Span::default())),
                Span::default(),
                Default::default(),
            );

            let result = infix.solve(&mut Compiler::default(), &mut Scope::new());
            assert!(result.is_err());
        }
    }
}
