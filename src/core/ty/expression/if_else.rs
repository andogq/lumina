use super::*;

impl parse_ast::If {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<If, TyError> {
        // Make sure the condition is correctly typed
        let condition = self.condition.ty_solve(ctx)?;
        let condition_ty = condition.get_ty_info();
        if !matches!(condition_ty.ty, Ty::Boolean) {
            return Err(TyError::Mismatch(Ty::Boolean, condition_ty.ty.clone()));
        }

        let success = self.success.ty_solve(ctx)?;
        let otherwise = self
            .otherwise
            .map(|otherwise| otherwise.ty_solve(ctx))
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
                    .map(|otherwise| otherwise.ty_info.return_ty)
                    .flatten(),
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
