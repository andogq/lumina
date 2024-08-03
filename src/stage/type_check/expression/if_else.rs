use crate::util::scope::Scope;

use super::*;

impl parse_ast::If {
    pub fn ty_solve(self, ctx: &mut impl TypeCheckCtx, scope: &mut Scope) -> Result<If, TyError> {
        // Make sure the condition is correctly typed
        let condition = self.condition.ty_solve(ctx, scope)?;
        let condition_ty = condition.get_ty_info();
        if !condition_ty.ty.check(&Ty::Boolean) {
            return Err(TyError::Mismatch(Ty::Boolean, condition_ty.ty));
        }

        let success = self.success.ty_solve(ctx, scope)?;
        let otherwise = self
            .otherwise
            .map(|otherwise| otherwise.ty_solve(ctx, scope))
            .transpose()?;

        let ty_info = TyInfo::try_from((
            // Branches must have the same type
            [
                success.ty_info.ty,
                otherwise
                    .as_ref()
                    .map(|otherwise| otherwise.ty_info.ty)
                    .unwrap_or(Ty::Unit),
            ],
            // Any potential place for a return statement must be accounted for
            [
                condition_ty.return_ty,
                success.ty_info.return_ty,
                otherwise
                    .as_ref()
                    .and_then(|otherwise| otherwise.ty_info.return_ty),
            ],
        ))?;

        Ok(If {
            ty_info,
            condition: Box::new(condition),
            success,
            otherwise,
            span: self.span,
        })
    }
}
