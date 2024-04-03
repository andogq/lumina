use crate::core::ast::{symbol::Symbol, ty::Ty};

use super::Block;

pub struct Function {
    pub name: Symbol,
    pub parameters: Vec<(Symbol, Ty)>,
    pub return_ty: Option<Ty>,
    pub body: Block,
}
