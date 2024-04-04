use crate::{core::symbol::Symbol, util::source::Span};

pub struct Ident {
    pub span: Span,
    pub name: Symbol,
}
