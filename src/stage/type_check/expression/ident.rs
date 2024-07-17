use super::*;

impl parse_ast::Ident {
    pub fn ty_solve(
        self,
        _ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<Ident, TyError> {
        Ok(Ident {
            ty_info: TyInfo {
                ty: scope
                    .resolve(self.name)
                    .map(|(_, ty)| ty)
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
        repr::ast::untyped::Ident,
        stage::type_check::ctx::{MockTypeCheckCtx, Scope},
        util::source::Span,
    };

    use super::expression::Ty;

    #[test]
    fn ident_present() {
        // Set up a reference symbol
        let symbol = Symbol::try_from_usize(0).unwrap();

        // Create a scope and add the symbol to it
        let mut scope = Scope::new();
        scope.register(symbol, Ty::Int);

        let i = Ident::new(symbol, Span::default());

        // Run the type solve
        let ty_info = i
            .ty_solve(&mut MockTypeCheckCtx::new(), &mut scope)
            .unwrap()
            .ty_info;

        assert_eq!(ty_info.ty, Ty::Int);
        assert_eq!(ty_info.return_ty, None);
    }

    #[test]
    fn ident_infer_missing() {
        let i = Ident::new(Symbol::try_from_usize(0).unwrap(), Span::default());

        let result = i.ty_solve(&mut MockTypeCheckCtx::new(), &mut Scope::new());

        assert!(result.is_err());
    }
}
