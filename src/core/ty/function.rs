use std::collections::HashMap;

use crate::core::ast::Function;

use super::{InferTy, TyError};

impl Function {
    pub fn check(&self) -> Result<(), TyError> {
        let inferred_ty = self.body.return_ty(&mut HashMap::new())?;
        if self.return_ty == inferred_ty {
            Ok(())
        } else {
            Err(TyError::Return {
                expected: self.return_ty,
                found: inferred_ty,
            })
        }
    }
}
