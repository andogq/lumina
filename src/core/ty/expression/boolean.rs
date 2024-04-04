use std::collections::HashMap;

use crate::core::{
    ast::Boolean,
    ty::{InferTy, Symbol, Ty, TyError},
};

impl InferTy for Boolean {
    fn infer(&self, _symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        Ok(Ty::Boolean)
    }

    fn return_ty(&self, _symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        Ok(None)
    }
}

#[cfg(test)]
mod test_boolean {
    use crate::util::source::Span;

    use super::*;

    #[test]
    fn boolean_infer() {
        assert_eq!(
            Boolean {
                span: Span::default(),
                value: false
            }
            .infer(&mut HashMap::new())
            .unwrap(),
            Ty::Boolean
        );
    }

    #[test]
    fn boolean_return() {
        assert_eq!(
            Boolean {
                span: Span::default(),
                value: false
            }
            .return_ty(&mut HashMap::new())
            .unwrap(),
            None,
        );
    }
}
