use ctx::TypeCheckCtx;

use super::*;

impl parse_ast::Program {
    pub fn ty_solve(self, ctx: &mut impl TypeCheckCtx) -> Result<Program, TyError> {
        // Main function must return int
        if self.main.return_ty != Ty::Int {
            return Err(TyError::Mismatch(Ty::Int, self.main.return_ty));
        }

        ctx.register_function(self.main.name, FunctionSignature::from(&self.main));

        // Pre-register all functions
        self.functions.iter().for_each(|function| {
            ctx.register_function(function.name, FunctionSignature::from(function));
        });

        // Make sure the type of the function is correct
        let main = self.main.ty_solve(ctx)?;

        let functions = self
            .functions
            .into_iter()
            .map(|function| function.ty_solve(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program {
            main,
            functions,
            span: self.span,

            // TODO: This should probably be migrated into something
            symbols: self.symbols,
        })
    }
}
