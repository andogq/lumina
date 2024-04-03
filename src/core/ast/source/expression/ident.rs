use crate::{core::ast::symbol::Symbol, util::source::Span};

pub struct Ident {
    pub span: Span,
    pub name: Symbol,
}
