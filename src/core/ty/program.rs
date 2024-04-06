use crate::core::ast::Program;

use super::{Ty, TyError};

impl Program {
    pub fn type_check(&self) -> Result<(), TyError> {
        // Main function must return int
        if !matches!(self.main.return_ty, Some(Ty::Int)) {
            return Err(TyError::Return {
                expected: Some(Ty::Int),
                found: self.main.return_ty,
            });
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
