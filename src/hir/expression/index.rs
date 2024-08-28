use super::*;

ast_node! {
    Index<M> {
        value: M::IdentIdentifier,
        index: Box<Expression<M>>,
        span,
        ty_info,
    }
}

impl SolveType for Index<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Ensure the inner parts are correct
        let index = self.index.solve(compiler, state)?;

        // Ensure that the index can be used as an index
        let index_ty = index.get_ty_info().ty.clone();
        if index_ty != Ty::Int {
            return Err(TyError::Mismatch(index_ty, Ty::Int));
        }

        let (value, ty) = state
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
