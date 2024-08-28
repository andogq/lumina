use super::*;

ast_node! {
    Block<M> {
        statements: Vec<Statement<M>>,
        span,
        ty_info,
    }
}

impl SolveType for Block<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        // Enter a new scope
        let block_scope = state.enter();

        let statements = self
            .statements
            .into_iter()
            .map(|statement| statement.solve(compiler, state))
            .collect::<Result<Vec<_>, _>>()?;

        let ty_info = TyInfo::try_from((
            // Type of this block will be the implicit return of the last block
            statements
                .last()
                // The block can only inherit the type of an expression statement
                // .filter(|s| {
                //     matches!(
                //         s,
                //         Statement::Expression(ExpressionStatement {
                //             implicit_return: true,
                //             ..
                //         })
                //     )
                // })
                .map(|s| s.get_ty_info().ty.clone())
                .unwrap_or(Ty::Unit),
            statements
                .iter()
                .map(|statement| statement.get_ty_info().return_ty.clone()),
        ))?;

        // Leave a scope
        assert_eq!(
            block_scope,
            state.leave(),
            "ensure the scope that is left was the same that was entered"
        );

        Ok(Block {
            span: self.span,
            statements,
            ty_info,
        })
    }
}
