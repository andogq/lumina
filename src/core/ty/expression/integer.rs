use std::collections::HashMap;

use crate::core::{
    ast::parse_ast::*,
    symbol::Symbol,
    ty::{InferTy, Ty, TyError},
};

impl InferTy for Integer {
    fn infer(&self, _symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        Ok(Ty::Int)
    }

    fn return_ty(&self, _symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        Ok(None)
    }
}

#[cfg(test)]
mod test_integer {
    use crate::util::source::Span;

    use super::*;

    #[test]
    fn integer_infer() {
        assert_eq!(
            Integer::new(0, Span::default())
                .infer(&mut HashMap::new())
                .unwrap(),
            Ty::Int
        );
    }

    #[test]
    fn integer_return() {
        assert_eq!(
            Integer::new(0, Span::default())
                .return_ty(&mut HashMap::new())
                .unwrap(),
            None,
        );
    }
}
