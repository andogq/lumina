use super::*;

impl parse_ast::Ident {
    pub fn ty_solve(self, ctx: &mut FnCtx) -> Result<Ident, TyError> {
        Ok(Ident {
            ty_info: TyInfo {
                ty: ctx
                    .scope
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

    use string_interner::Symbol;

    use crate::{
        core::{parse::ast::*, ty::Ty},
        util::source::Span,
    };

    use super::expression::{FnCtx, TyError, TyInfo};

    fn run(i: Ident, ident: bool) -> Result<TyInfo, TyError> {
        let mut fn_ctx = FnCtx::mock();
        if ident {
            fn_ctx
                .scope
                .insert(Symbol::try_from_usize(0).unwrap(), Ty::Int);
        }

        Ok(i.ty_solve(&mut fn_ctx)?.ty_info)
    }

    #[test]
    fn ident_present() {
        let ty_info = run(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default()),
            true,
        )
        .unwrap();

        assert_eq!(ty_info.ty, Ty::Int);
        assert_eq!(ty_info.return_ty, None);
    }

    #[test]
    fn ident_infer_missing() {
        assert!(run(
            Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default()),
            false,
        )
        .is_err());
    }
}
