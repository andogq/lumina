use crate::core::{
    ast::{parse_ast, ty_ast::*},
    ty::{Ty, TyCtx, TyError},
};

impl InfixOperation {
    /// Determine the resulting type if this operator is applied to the provided parameters.
    fn result_ty(&self, left: &Ty, right: &Ty) -> Result<Ty, TyError> {
        use InfixOperation::*;

        match (self, left, right) {
            (Plus(_), Ty::Int, Ty::Int) => Ok(Ty::Int),
            (Eq(_) | NotEq(_), left, right) if left == right => Ok(Ty::Boolean),
            (_, left, right) => Err(TyError::Mismatch(left.clone(), right.clone())),
        }
    }
}

impl parse_ast::Infix {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<Infix, TyError> {
        let left = self.left.ty_solve(ctx)?;
        let right = self.right.ty_solve(ctx)?;

        let left_ty_info = left.get_ty_info();
        let right_ty_info = right.get_ty_info();

        let ty_info = TyInfo::try_from((
            // Resulting type is whatever the infix operator results in
            self.operation
                .result_ty(&left_ty_info.ty, &right_ty_info.ty)?,
            [left_ty_info.return_ty, right_ty_info.return_ty],
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
    use crate::{
        core::{ast::parse_ast::*, ty::Ty},
        util::source::Span,
    };

    #[test]
    fn infer_same() {
        // 0 + 0
        let infix = Infix::new(
            Box::new(Expression::integer(0, Span::default())),
            InfixOperation::plus(),
            Box::new(Expression::integer(0, Span::default())),
            Span::default(),
        );

        assert_eq!(
            infix.ty_solve(&mut Default::default()).unwrap().ty_info.ty,
            Ty::Int
        );
    }
    #[test]
    fn infer_different() {
        // 0 + false
        let infix = Infix::new(
            Box::new(Expression::integer(0, Span::default())),
            InfixOperation::plus(),
            Box::new(Expression::boolean(false, Span::default())),
            Span::default(),
        );

        assert!(infix.ty_solve(&mut Default::default()).is_err());
    }

    #[test]
    fn return_same() {
        // 0 + 0
        let infix = Infix::new(
            Box::new(Expression::integer(0, Span::default())),
            InfixOperation::plus(),
            Box::new(Expression::integer(0, Span::default())),
            Span::default(),
        );

        assert_eq!(
            infix
                .ty_solve(&mut Default::default())
                .unwrap()
                .ty_info
                .return_ty,
            None
        );
    }
    #[test]
    fn return_different() {
        // 0 + 0
        let infix = Infix::new(
            Box::new(Expression::integer(0, Span::default())),
            InfixOperation::plus(),
            Box::new(Expression::integer(0, Span::default())),
            Span::default(),
        );

        assert_eq!(
            infix
                .ty_solve(&mut Default::default())
                .unwrap()
                .ty_info
                .return_ty,
            None
        );
    }
}
