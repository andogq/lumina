use super::*;

use std::hash::Hash;

ast_node! {
    Ident<M> {
        binding: M::IdentIdentifier,
        span,
        ty_info,
    }
}

impl SolveType for Ident<UntypedAstMetadata> {
    type State = Scope;

    fn solve(
        self,
        _compiler: &mut crate::compiler::Compiler,
        state: &mut Self::State,
    ) -> Result<Self::Typed, crate::stage::type_check::TyError> {
        let (binding, ty) = state
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

impl<M: AstMetadata<IdentIdentifier: Hash>> Hash for Ident<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.binding.hash(state);
    }
}

impl<M: AstMetadata<IdentIdentifier: PartialEq>> PartialEq for Ident<M>
where
    M::IdentIdentifier: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding
    }
}

impl<M: AstMetadata> Eq for Ident<M> where M::IdentIdentifier: Eq {}

#[cfg(test)]
mod test_ident {
    use string_interner::Symbol;

    use super::*;

    #[test]
    fn ident_present() {
        // Set up a reference symbol
        let symbol = Symbol::try_from_usize(0).unwrap();

        // Create a scope and add the symbol to it
        let mut scope = Scope::new();
        scope.register(symbol, Ty::Int);

        let i = Ident::new(symbol, Span::default(), Default::default());

        // Run the type solve
        let ty_info = i
            .solve(&mut Compiler::default(), &mut scope)
            .unwrap()
            .ty_info;

        assert_eq!(ty_info.ty, Ty::Int);
        assert_eq!(ty_info.return_ty, None);
    }

    #[test]
    fn ident_infer_missing() {
        let i = Ident::new(
            Symbol::try_from_usize(0).unwrap(),
            Span::default(),
            Default::default(),
        );

        let result = i.solve(&mut Compiler::default(), &mut Scope::new());

        assert!(result.is_err());
    }
}
