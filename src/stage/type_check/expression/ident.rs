use crate::util::scope::Scope;

use super::*;

impl parse_ast::Ident {
    pub fn ty_solve(
        self,
        _ctx: &mut impl TypeCheckCtx,
        scope: &mut Scope,
    ) -> Result<Ident, TyError> {
        let (binding, ty) = scope
            .resolve(self.binding)
            .ok_or(TyError::SymbolNotFound(self.binding))?;

        Ok(Ident {
            ty_info: TyInfo {
                ty,
                return_ty: None,
            },
            binding,
            span: self.span,
        })
    }
}

#[cfg(test)]
mod test_ident {
    use string_interner::Symbol;

    use crate::{
        repr::ast::untyped::Ident,
        stage::type_check::ctx::MockTypeCheckCtx,
        util::{scope::Scope, source::Span},
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
