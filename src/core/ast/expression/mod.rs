mod block;
mod boolean;
mod ident;
mod if_else;
mod infix;
mod integer;

pub use block::*;
pub use boolean::*;
pub use ident::*;
pub use if_else::*;
pub use infix::*;
pub use integer::*;

use crate::{
    core::symbol::Symbol,
    util::source::{Span, Spanned},
};

use super::Statement;

#[derive(Debug, Clone)]
pub enum Expression {
    Infix(Infix),
    Integer(Integer),
    Boolean(Boolean),
    Ident(Ident),
    Block(Block),
    If(If),
}

impl Expression {
    pub fn infix(left: Expression, operation: InfixOperation, right: Expression) -> Self {
        Self::Infix(Infix::new(left, operation, right))
    }

    pub fn integer(value: i64) -> Self {
        Self::Integer(Integer::new(value))
    }

    pub fn boolean(value: bool) -> Self {
        Self::Boolean(Boolean::new(value))
    }

    pub fn ident(name: Symbol) -> Self {
        Self::Ident(Ident::new(name))
    }

    pub fn block(statements: &[Statement]) -> Self {
        Self::Block(Block::new(statements))
    }

    pub fn _if(condition: Expression, success: Block, otherwise: Option<Block>) -> Self {
        Self::If(If::new(condition, success, otherwise))
    }
}

impl Spanned for Expression {
    fn span(&self) -> &Span {
        match self {
            Expression::Infix(s) => s.span(),
            Expression::Integer(s) => s.span(),
            Expression::Boolean(s) => s.span(),
            Expression::Ident(s) => s.span(),
            Expression::Block(s) => s.span(),
            Expression::If(s) => s.span(),
        }
    }
}
