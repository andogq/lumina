use std::hash::Hash;

use crate::{ast_node, repr::ast::base::AstMetadata};

ast_node! {
    Ident<M> {
        binding: M::IdentIdentifier,
        span,
        ty_info,
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
