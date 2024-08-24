use crate::{compiler::Compiler, util::scope::Scope};

use super::*;

impl parse_ast::Assign {
    pub fn ty_solve(self, compiler: &mut Compiler, scope: &mut Scope) -> Result<Assign, TyError> {
        // Work out what type the variable has to be
        let (binding, ty) = scope
            .resolve(self.binding)
            .ok_or(TyError::SymbolNotFound(self.binding))?;

        let value = self.value.ty_solve(compiler, scope)?;

        let value_ty = value.get_ty_info().ty.clone();

        if value_ty != ty {
            return Err(TyError::Mismatch(ty, value_ty));
        }

        Ok(Assign {
            binding,
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: value.get_ty_info().return_ty.clone(),
            },
            value: Box::new(value),
            span: self.span,
        })
    }
}
