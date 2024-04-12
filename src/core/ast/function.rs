use crate::{
    core::{symbol::Symbol, ty::Ty},
    util::source::{Span, Spanned},
};

use super::Block;

pub struct Function {
    pub span: Span,
    pub name: Symbol,
    pub parameters: Vec<(Symbol, Ty)>,
    pub return_ty: Ty,
    pub body: Block,
}

impl Spanned for Function {
    fn span(&self) -> &Span {
        &self.span
    }
}
