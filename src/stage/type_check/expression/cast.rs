use super::*;

impl parse_ast::Cast {
    pub fn ty_solve(self, compiler: &mut Compiler, scope: &mut Scope) -> Result<Cast, TyError> {
        let value = self.value.ty_solve(compiler, scope)?;

        // Make sure that the value can be cast to the desired type
        match (value.get_ty_info().ty, self.target_ty) {
            // Unsigned integer can become signed
            (Ty::Uint, Ty::Int) => (),
            // Signed integer can loose sign
            (Ty::Int, Ty::Uint) => (),
            (lhs, rhs) => return Err(TyError::Cast(lhs, rhs)),
        }

        Ok(Cast {
            target_ty: self.target_ty,
            span: self.span,
            ty_info: TyInfo {
                ty: self.target_ty,
                return_ty: value.get_ty_info().return_ty,
            },
            value: Box::new(value),
        })
    }
}
