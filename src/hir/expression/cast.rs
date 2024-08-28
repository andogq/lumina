use super::*;

ast_node! {
    Cast<M> {
        value: Box<Expression<M>>,
        target_ty: Ty,
        span,
        ty_info,
    }
}

impl SolveType for Cast<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        let value = self.value.solve(compiler, state)?;

        // Make sure that the value can be cast to the desired type
        match (value.get_ty_info().ty.clone(), self.target_ty.clone()) {
            // Unsigned integer can become signed
            (Ty::Uint, Ty::Int) => (),
            // Signed integer can loose sign
            (Ty::Int, Ty::Uint) => (),
            (lhs, rhs) => return Err(TyError::Cast(lhs, rhs)),
        }

        Ok(Cast {
            target_ty: self.target_ty.clone(),
            span: self.span,
            ty_info: TyInfo {
                ty: self.target_ty.clone(),
                return_ty: value.get_ty_info().return_ty.clone(),
            },
            value: Box::new(value),
        })
    }
}
