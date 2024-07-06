use std::collections::HashMap;

use crate::core::{
    ast::If,
    symbol::Symbol,
    ty::{InferTy, Ty, TyError},
};

impl InferTy for If<()> {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        let condition_ty = self.condition.infer(symbols)?;
        if condition_ty != Ty::Boolean {
            return Err(TyError::Mismatch(Ty::Boolean, condition_ty));
        }

        match (
            self.success.infer(symbols)?,
            self.otherwise
                .as_ref()
                .map(|otherwise| otherwise.infer(symbols))
                .transpose()?,
        ) {
            (success, Some(otherwise)) if success == otherwise => Ok(success),
            (Ty::Unit, None) => Ok(Ty::Unit),

            (success, Some(otherwise)) => Err(TyError::Mismatch(success, otherwise)),
            (success, None) => Err(TyError::Mismatch(success, Ty::Unit)),
        }
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        let condition = self.condition.return_ty(symbols)?;
        let success = self.success.return_ty(symbols)?;

        // TODO: Better reporting of conflicting return types
        if condition != success {
            return Err(TyError::Return {
                expected: condition,
                found: success,
            });
        }

        if let Some(otherwise) = &self.otherwise {
            let otherwise = otherwise.return_ty(symbols)?;

            if condition != otherwise {
                return Err(TyError::Return {
                    expected: condition,
                    found: otherwise,
                });
            }
        }

        Ok(None)
    }
}
