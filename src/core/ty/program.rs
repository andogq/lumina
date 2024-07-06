use super::*;

impl parse_ast::Program {
    pub fn ty_solve(self) -> Result<Program, TyError> {
        // Main function must return int
        if self.main.return_ty != Ty::Int {
            return Err(TyError::Mismatch(Ty::Int, self.main.return_ty));
        }

        let mut ctx = TyCtx::default();

        // Make sure the type of the function is correct
        let main = self.main.ty_solve(&mut ctx)?;

        let functions = self
            .functions
            .into_iter()
            .map(|function| function.ty_solve(&mut ctx))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program {
            main,
            functions,
            span: self.span,

            // TODO: This should probably be migrated into something
            symbols: self.symbols,

            // WARN: Program should not have type
            ty_info: TyInfo {
                ty: Ty::Unit,
                return_ty: Some(Ty::Unit),
            },
        })
    }
}
