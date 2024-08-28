use super::*;

#[derive(Debug, Clone)]
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
    type Error = ();

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
            _ => Err(()),
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
