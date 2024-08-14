use crate::util::scope::Scope;

use super::*;

impl InfixOperation {
    /// Determine the resulting type if this operator is applied to the provided parameters.
    fn result_ty(&self, left: &Ty, right: &Ty) -> Result<Ty, TyError> {
        use InfixOperation::*;

        match (self, left, right) {
            (Plus | Minus, Ty::Int, Ty::Int) => Ok(Ty::Int),
            (Eq | NotEq, left, right) if left.check(right) => Ok(Ty::Boolean),
            (And | Or, Ty::Boolean, Ty::Boolean) => Ok(Ty::Boolean),
            (_, left, right) => Err(TyError::Mismatch(*left, *right)),
        }
    }
}

impl parse_ast::Infix {
    pub fn ty_solve(
        self,
        ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<Infix, TyError> {
        let left = self.left.ty_solve(ctx, scope)?;
        let right = self.right.ty_solve(ctx, scope)?;

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
        repr::{ast::untyped::*, ty::Ty},
        stage::type_check::ctx::MockTypeCheckCtx,
        util::{scope::Scope, span::Span},
    };

    #[test]
    fn infix_same() {
        // 0 + 0
        let infix = Infix::new(
            Box::new(Expression::integer(0, Span::default())),
            InfixOperation::plus(),
            Box::new(Expression::integer(0, Span::default())),
            Span::default(),
        );

        let ty_info = infix
            .ty_solve(&mut MockTypeCheckCtx::new(), &mut Scope::new())
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
        );

        let result = infix.ty_solve(&mut MockTypeCheckCtx::new(), &mut Scope::new());
        assert!(result.is_err());
    }
}
