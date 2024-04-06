use std::collections::HashMap;

use crate::core::ast::Function;

use super::{InferTy, Ty, TyError};

impl Function {
    pub fn check(&self) -> Result<(), TyError> {
        // Block can implicitly evaluate to a value
        let inferred_ty = self.body.infer(&mut HashMap::new())?;

        // Block can have return statements
        let return_ty = self.body.return_ty(&mut HashMap::new())?;

        // Make sure explicit returns are correct
        if let Some(return_ty) = return_ty {
            if return_ty != self.return_ty.unwrap_or(Ty::Unit) {
                return Err(TyError::Return {
                    expected: self.return_ty,
                    found: Some(return_ty),
                });
            }
        }

        if !matches!(inferred_ty, Ty::Unit) && inferred_ty != self.return_ty.unwrap_or(Ty::Unit) {
            return Err(TyError::Return {
                expected: self.return_ty,
                found: Some(inferred_ty),
            });
        }

        Ok(())
    }
}
