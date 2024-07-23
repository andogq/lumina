use std::hash::Hash;

use crate::ast_node;

ast_node! {
    typed struct Ident<TyInfo, IdentIdentifier> {
        binding: IdentIdentifier,
    }
}

impl<TyInfo, IdentIdentifier: Hash> Hash for Ident<TyInfo, IdentIdentifier> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.binding.hash(state);
    }
}

impl<TyInfo, IdentIdentifier: PartialEq> PartialEq for Ident<TyInfo, IdentIdentifier> {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding
    }
}

impl<TyInfo, IdentIdentifier: Eq> Eq for Ident<TyInfo, IdentIdentifier> {}
