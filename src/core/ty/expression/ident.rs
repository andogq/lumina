use super::*;

impl parse_ast::Ident {
    pub fn ty_solve(self, ctx: &mut TyCtx) -> Result<Ident, TyError> {
        Ok(Ident {
            ty_info: TyInfo {
                ty: ctx
                    .symbols
                    .get(&self.name)
                    .cloned()
                    .ok_or(TyError::SymbolNotFound(self.name))?,
                return_ty: None,
            },
            name: self.name,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test_ident {
    use std::collections::HashMap;

    use string_interner::Symbol;

    use crate::{
        core::{
            parse::ast::*,
            ty::{Ty, TyCtx},
        },
        util::source::Span,
    };

    #[test]
    fn ident_infer() {
        assert_eq!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .ty_solve(&mut TyCtx {
                    symbols: HashMap::from_iter([(Symbol::try_from_usize(0).unwrap(), Ty::Int)])
                })
                .unwrap()
                .ty_info
                .ty,
            Ty::Int,
        );
    }

    #[test]
    fn ident_return() {
        assert_eq!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .ty_solve(&mut TyCtx {
                    symbols: HashMap::from_iter([(Symbol::try_from_usize(0).unwrap(), Ty::Int)])
                })
                .unwrap()
                .ty_info
                .return_ty,
            None,
        );
    }

    #[test]
    fn ident_infer_missing() {
        assert!(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default())
                .ty_solve(&mut Default::default())
                .is_err(),
        );
    }
}
