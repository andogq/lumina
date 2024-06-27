use std::hash::Hash;

use crate::{
    core::symbol::Symbol,
    util::source::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct Ident {
    pub span: Span,
    pub name: Symbol,
}

impl Ident {
    pub fn new(name: Symbol) -> Self {
        Self {
            span: Span::default(),
            name,
        }
    }
}

impl Spanned for Ident {
    fn span(&self) -> &Span {
        &self.span
    }
}

impl Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Ident {}
