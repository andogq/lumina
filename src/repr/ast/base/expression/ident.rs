use std::hash::Hash;

use crate::{ast_node, util::symbol_map::interner_symbol_map::Symbol};

ast_node! {
    typed struct Ident<TyInfo> {
        name: Symbol,
    }
}

impl<TyInfo> Hash for Ident<TyInfo> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<TyInfo> PartialEq for Ident<TyInfo> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<TyInfo> Eq for Ident<TyInfo> {}
