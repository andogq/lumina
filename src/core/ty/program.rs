use crate::core::ast::Program;

use super::{Ty, TyError};

impl Program {
    pub fn type_check(&self) -> Result<(), TyError> {
        // Main function must return int
        if self.main.return_ty != Ty::Int {
            return Err(TyError::Mismatch(Ty::Int, self.main.return_ty));
        }

        // Make sure the type of the function is correct
        self.main.check()?;

        for function in &self.functions {
            // Type check all the functions in the program
            function.check()?;
        }

        Ok(())
    }
}
