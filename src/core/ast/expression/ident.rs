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
