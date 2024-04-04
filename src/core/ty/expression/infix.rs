use std::collections::HashMap;

use crate::core::{
    ast::Infix,
    ty::{InferTy, Symbol, Ty, TyError},
};

impl InferTy for Infix {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        let left_ty = self.left.infer(symbols)?;
        let right_ty = self.right.infer(symbols)?;

        if left_ty == right_ty {
            Ok(left_ty)
        } else {
            Err(TyError::Mismatch(left_ty, right_ty))
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
    use crate::{
        core::ast::{Boolean, Expression, InfixOperation, Integer},
        util::source::Span,
    };

    use super::*;

    #[test]
    fn infer_same() {
        let infix = Infix {
            left: Box::new(Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            })),
            operation: InfixOperation::Plus(Span::default()),
            right: Box::new(Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            })),
        };

        assert_eq!(infix.infer(&mut HashMap::new()).unwrap(), Ty::Int);
    }
    #[test]
    fn infer_different() {
        let infix = Infix {
            left: Box::new(Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            })),
            operation: InfixOperation::Plus(Span::default()),
            right: Box::new(Expression::Boolean(Boolean {
                span: Span::default(),
                value: false,
            })),
        };

        assert!(infix.infer(&mut HashMap::new()).is_err());
    }

    #[test]
    fn return_same() {
        let infix = Infix {
            left: Box::new(Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            })),
            operation: InfixOperation::Plus(Span::default()),
            right: Box::new(Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            })),
        };

        assert_eq!(infix.return_ty(&mut HashMap::new()).unwrap(), None);
    }
    #[test]
    fn return_different() {
        let infix = Infix {
            left: Box::new(Expression::Integer(Integer {
                span: Span::default(),
                value: 0,
            })),
            operation: InfixOperation::Plus(Span::default()),
            right: Box::new(Expression::Boolean(Boolean {
                span: Span::default(),
                value: false,
            })),
        };

        assert_eq!(infix.return_ty(&mut HashMap::new()).unwrap(), None);
    }
}
