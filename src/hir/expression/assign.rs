use super::*;

ast_node! {
    Assign<M> {
        binding: M::IdentIdentifier,
        value: Box<Expression<M>>,
        span,
        ty_info,
    }
}

impl SolveType for Assign<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Work out what type the variable has to be
        let (binding, ty) = state
            .resolve(self.binding)
            .ok_or(TyError::SymbolNotFound(self.binding))?;

        let value = self.value.solve(compiler, state)?;

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
