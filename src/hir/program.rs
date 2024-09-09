use crate::ty::FunctionSignature;

use super::*;

ast_node! {
    Program<M> {
        functions: Vec<Function<M>>,
        main: Function<M>,
        span,
    }
}

impl SolveType for Program<UntypedAstMetadata> {
    type State = ();

    fn solve(
        self,
        compiler: &mut crate::compiler::Compiler,
        _state: &mut Self::State,
    ) -> Result<Self::Typed, TyError> {
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
        let main = self.main.solve(compiler, &mut ())?;

        let functions = self
            .functions
            .into_iter()
            .map(|function| function.solve(compiler, &mut ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program {
            main,
            functions,
            span: self.span,
        })
    }
}
