use std::collections::HashMap;

use crate::core::{
    ast::parse_ast::*,
    ty::{InferTy, Symbol, Ty, TyError},
};

impl InferTy for Ident {
    fn infer(&self, symbols: &mut HashMap<Symbol, Ty>) -> Result<Ty, TyError> {
        symbols
            .get(&self.name)
            .ok_or(TyError::SymbolNotFound(self.name))
            .cloned()
    }

    fn return_ty(&self, _symbols: &mut HashMap<Symbol, Ty>) -> Result<Option<Ty>, TyError> {
        Ok(None)
    }
}

#[cfg(test)]
mod test_ident {
    use string_interner::Symbol;

    use crate::util::source::Span;

    use super::*;

    #[test]
    fn ident_infer() {
        assert_eq!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .infer(&mut HashMap::from_iter([(
                    Symbol::try_from_usize(0).unwrap(),
                    Ty::Int
                )]))
                .unwrap(),
            Ty::Int,
        );
    }

    #[test]
    fn ident_return() {
        assert_eq!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .return_ty(&mut HashMap::from_iter([(
                    Symbol::try_from_usize(0).unwrap(),
                    Ty::Int
                )]))
                .unwrap(),
            None,
        );
    }

    #[test]
    fn ident_infer_missing() {
        assert!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .infer(&mut HashMap::new())
                .is_err(),
        );
    }

    #[test]
    fn ident_return_missing() {
        assert_eq!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .return_ty(&mut HashMap::from_iter([(
                    Symbol::try_from_usize(0).unwrap(),
                    Ty::Int
                )]))
                .unwrap(),
            None,
        );
    }
}
