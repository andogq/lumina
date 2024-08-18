use crate::compiler::Compiler;

use super::*;

impl parse_ast::Program {
    pub fn ty_solve(self, compiler: &mut Compiler) -> Result<Program, TyError> {
        // Main function must return int
        if !self.main.return_ty.check(&Ty::Int) {
            return Err(TyError::Mismatch(Ty::Int, self.main.return_ty));
        }

        compiler
            .functions
            .register(self.main.name, FunctionSignature::from(&self.main));

        // Pre-register all functions
        self.functions.iter().for_each(|function| {
            compiler
                .functions
                .register(function.name, FunctionSignature::from(function));
        });

        // Make sure the type of the function is correct
        let main = self.main.ty_solve(compiler)?;

        let functions = self
            .functions
            .into_iter()
            .map(|function| function.ty_solve(compiler))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program {
            main,
            functions,
            span: self.span,
        })
    }
}
