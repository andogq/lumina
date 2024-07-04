use std::collections::HashMap;

use crate::core::{
    ast::{Infix, InfixOperation},
    ty::{InferTy, Symbol, Ty, TyError},
};

impl InferTy for Infix {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        let left_ty = self.left.infer(symbols)?;
        let right_ty = self.right.infer(symbols)?;

        match self.operation {
            InfixOperation::Plus(_) => {
                if left_ty == right_ty {
                    Ok(left_ty)
                } else {
                    Err(TyError::Mismatch(left_ty, right_ty))
                }
            }
            InfixOperation::Eq(_) | InfixOperation::NotEq(_) => {
                if left_ty == right_ty {
                    Ok(Ty::Boolean)
                } else {
                    Err(TyError::Mismatch(left_ty, right_ty))
                }
            }
        }
    }

    fn return_ty(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        let left_return_ty = self.left.return_ty(symbols)?;
        let right_return_ty = self.right.return_ty(symbols)?;

        match (left_return_ty, right_return_ty) {
            (Some(left), Some(right)) if left == right => Ok(Some(left)),
            (Some(left), Some(right)) => Err(TyError::Mismatch(left, right)),
            (Some(ty), None) | (None, Some(ty)) => Ok(Some(ty)),
            (None, None) => Ok(None),
        }
    }
}
#[cfg(test)]
mod test_infix {
    use crate::core::ast::{Expression, InfixOperation};

    use super::*;

    #[test]
    fn infer_same() {
        // 0 + 0
        let infix = Infix::new(
            Expression::integer(0),
            InfixOperation::plus(),
            Expression::integer(0),
        );

        assert_eq!(infix.infer(&mut HashMap::new()).unwrap(), Ty::Int);
    }
    #[test]
    fn infer_different() {
        // 0 + false
        let infix = Infix::new(
            Expression::integer(0),
            InfixOperation::plus(),
            Expression::boolean(false),
        );

        assert!(infix.infer(&mut HashMap::new()).is_err());
    }

    #[test]
    fn return_same() {
        // 0 + 0
        let infix = Infix::new(
            Expression::integer(0),
            InfixOperation::plus(),
            Expression::integer(0),
        );

        assert_eq!(infix.return_ty(&mut HashMap::new()).unwrap(), None);
    }
    #[test]
    fn return_different() {
        // 0 + 0
        let infix = Infix::new(
            Expression::integer(0),
            InfixOperation::plus(),
            Expression::integer(0),
        );

        assert_eq!(infix.return_ty(&mut HashMap::new()).unwrap(), None);
    }
}
