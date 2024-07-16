use super::*;

impl parse_ast::Call {
    pub fn ty_solve(self, ctx: &mut FnCtx) -> Result<Call, TyError> {
        // Determine the types of all the arguments
        let args = self
            .args
            .into_iter()
            .map(|arg| arg.ty_solve(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let ty_ctx = ctx.ty_ctx.borrow();

        // Compare the arguments to the function types
        let signature = ty_ctx
            .function_signatures
            .get(&self.name)
            .ok_or(TyError::SymbolNotFound(self.name))?;

        if args.len() != signature.arguments.len() {
            // TODO: Make new type error for when the function call has too many arguments
            panic!("too many arguments");
        }

        if !args
            .iter()
            .map(|arg| arg.get_ty_info().ty)
            .zip(&signature.arguments)
            .all(|(call, signature)| call == *signature)
        {
            // TODO: New type error when a parameter is the wrong type
            panic!("parameter wong type");
        }

        Ok(Call {
            ty_info: TyInfo::try_from((
                // Function call's resulting type will be whatever the function returns
                signature.return_ty,
                // Ensure all the return types from the arguments are correct
                args.iter().map(|arg| arg.get_ty_info().return_ty),
            ))?,
            // TODO: Resolve a symbol into a function
            name: todo!(), //self.name,
            args,
            span: self.span,
        })
    }
}
