use crate::util::scope::Scope;

use super::*;

impl parse_ast::If {
    pub fn ty_solve(self, compiler: &mut Compiler, scope: &mut Scope) -> Result<If, TyError> {
        // Make sure the condition is correctly typed
        let condition = self.condition.ty_solve(compiler, scope)?;
        let condition_ty = condition.get_ty_info();
        if !condition_ty.ty.check(&Ty::Boolean) {
            return Err(TyError::Mismatch(Ty::Boolean, condition_ty.ty.clone()));
        }

        let success = self.success.ty_solve(compiler, scope)?;
        let otherwise = self
            .otherwise
            .map(|otherwise| otherwise.ty_solve(compiler, scope))
            .transpose()?;

        let ty_info = TyInfo::try_from((
            // Branches must have the same type
            [
                success.ty_info.ty.clone(),
                otherwise
                    .as_ref()
                    .map(|otherwise| otherwise.ty_info.ty.clone())
                    .unwrap_or(Ty::Unit),
            ],
            // Any potential place for a return statement must be accounted for
            [
                condition_ty.return_ty.clone(),
                success.ty_info.return_ty.clone(),
                otherwise
                    .as_ref()
                    .and_then(|otherwise| otherwise.ty_info.return_ty.clone()),
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
