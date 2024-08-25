use super::*;

impl parse_ast::Index {
    pub fn ty_solve(self, compiler: &mut Compiler, scope: &mut Scope) -> Result<Index, TyError> {
        // Ensure the inner parts are correct
        let index = self.index.ty_solve(compiler, scope)?;

        // Ensure that the index can be used as an index
        let index_ty = index.get_ty_info().ty.clone();
        if index_ty != Ty::Int {
            return Err(TyError::Mismatch(index_ty, Ty::Int));
        }

        let (value, ty) = scope
            .resolve(self.value)
            .ok_or(TyError::SymbolNotFound(self.value))?;

        // Ensure the value is indexable
        let result_ty = if let Ty::Array {
            inner: inner_ty, ..
        } = ty
        {
            *inner_ty
        } else {
            return Err(TyError::Index(ty));
        };

        Ok(Index {
            value,
            span: self.span,
            ty_info: TyInfo {
                ty: result_ty,
                return_ty: index.get_ty_info().return_ty.clone(),
            },
            index: Box::new(index),
        })
    }
}
