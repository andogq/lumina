use super::*;

ast_node! {
    Loop<M> {
        body: Block<M>,
        span,
        ty_info,
    }
}

impl SolveType for Loop<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Type check the body
        let body = self.body.solve(compiler, state)?;

        // TODO: Temporary whilst can't break expression
        match body.ty_info.ty {
            Ty::Unit | Ty::Never => (),
            ty => {
                return Err(TyError::Mismatch(Ty::Unit, ty));
            }
        };

        Ok(Loop {
            ty_info: body.ty_info.clone(),
            body,
            span: self.span,
        })
    }
}
