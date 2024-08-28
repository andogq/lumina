use super::*;

ast_node! {
    Call<M> {
        name: M::FnIdentifier,
        args: Vec<Expression<M>>,
        span,
        ty_info,
    }
}

impl SolveType for Call<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
        // Determine the types of all the arguments
        let args = self
            .args
            .into_iter()
            .map(|arg| arg.solve(compiler, state))
            .collect::<Result<Vec<_>, _>>()?;

        // Compare the arguments to the function types
        let function_idx = compiler
            .functions
            .get_idx(self.name)
            .ok_or(TyError::SymbolNotFound(self.name))?;
        let signature = compiler
            .functions
            .get(function_idx)
            .expect("function must be defined")
            .get_signature();

        if args.len() != signature.arguments.len() {
            // TODO: Make new type error for when the function call has too many arguments
            panic!("too many arguments");
        }

        if !args
            .iter()
            .map(|arg| arg.get_ty_info().ty.clone())
            .zip(&signature.arguments)
            .all(|(call, signature)| call == *signature)
        {
            // TODO: New type error when a parameter is the wrong type
            panic!("parameter wong type");
        }

        Ok(Call {
            ty_info: TyInfo::try_from((
                // Function call's resulting type will be whatever the function returns
                signature.return_ty.clone(),
                // Ensure all the return types from the arguments are correct
                args.iter().map(|arg| arg.get_ty_info().return_ty.clone()),
            ))?,
            name: function_idx,
            args,
            span: self.span,
        })
    }
}