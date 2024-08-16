use crate::util::scope::Scope;

use super::*;

impl parse_ast::Call {
    pub fn ty_solve(self, compiler: &mut Compiler, scope: &mut Scope) -> Result<Call, TyError> {
        // Determine the types of all the arguments
        let args = self
            .args
            .into_iter()
            .map(|arg| arg.ty_solve(compiler, scope))
            .collect::<Result<Vec<_>, _>>()?;

        // Compare the arguments to the function types
        let function_idx = compiler
            .get_function_idx(self.name)
            .ok_or(TyError::SymbolNotFound(self.name))?;
        let signature = compiler
            .get_function(function_idx)
            .expect("function must be defined")
            .get_signature();

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
            name: function_idx,
            args,
            span: self.span,
        })
    }
}
