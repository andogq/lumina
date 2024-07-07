use super::*;

impl parse_ast::Call {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<Call, TyError> {
        // Determine the types of all the arguments
        let args = self
            .args
            .into_iter()
            .map(|arg| arg.ty_solve(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        // Compare the arguments to the function types
        let (function_args, function_return_ty) = ctx
            .functions
            .get(&self.name)
            .ok_or(TyError::SymbolNotFound(self.name))?;

        if args.len() != function_args.len() {
            // TODO: Make new type error for when the function call has too many arguments
            panic!("too many arguments");
        }

        if !args
            .iter()
            .map(|arg| arg.get_ty_info().ty)
            .zip(function_args)
            .all(|(call, signature)| call == *signature)
        {
            // TODO: New type error when a parameter is the wrong type
            panic!("parameter wong type");
        }

        Ok(Call {
            ty_info: TyInfo::try_from((
                // Function call's resulting type will be whatever the function returns
                *function_return_ty,
                // Ensure all the return types from the arguments are correct
                args.iter().map(|arg| arg.get_ty_info().return_ty),
            ))?,
            name: self.name,
            args,
            span: self.span,
        })
    }
}
