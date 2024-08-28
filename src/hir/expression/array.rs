use super::*;

ast_node! {
    Array<M> {
        init: Vec<Expression<M>>,
        span,
        ty_info,
    }
}

impl SolveType for Array<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Type check each of the init items
        let init = self
            .init
            .into_iter()
            .map(|i| i.solve(compiler, state))
            .collect::<Result<Vec<_>, _>>()?;

        // Make sure all of the init items agree on the type
        let ty_info = init
            .iter()
            .map(|i| i.get_ty_info().clone())
            .collect::<Result<TyInfo, _>>()?;

        Ok(Array {
            span: self.span,
            ty_info: TyInfo {
                ty: Ty::Array {
                    inner: Box::new(ty_info.ty),
                    size: init.len() as u32,
                },
                return_ty: ty_info.return_ty,
            },
            init,
        })
    }
}
