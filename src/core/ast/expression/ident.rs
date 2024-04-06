use crate::{core::symbol::Symbol, util::source::Span};

#[derive(Debug)]
pub struct Ident {
    pub span: Span,
    pub name: Symbol,
}
