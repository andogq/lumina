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
pub enum Expression<TyInfo> {
    Infix(Infix<TyInfo>),
    Integer(Integer<TyInfo>),
    Boolean(Boolean<TyInfo>),
    Ident(Ident<TyInfo>),
    Block(Block<TyInfo>),
    If(If<TyInfo>),
}

impl<TyInfo: Default> Expression<TyInfo> {
    pub fn infix(
        left: Expression<TyInfo>,
        operation: InfixOperation,
        right: Expression<TyInfo>,
    ) -> Self {
        let span = Spanned::span(&left).to(&right);
        Self::Infix(Infix::<TyInfo>::new(
            Box::new(left),
            operation,
            Box::new(right),
            span,
        ))
    }

    pub fn integer(value: i64, span: Span) -> Self {
        Self::Integer(Integer::<TyInfo>::new(value, span))
    }

    pub fn boolean(value: bool, span: Span) -> Self {
        Self::Boolean(Boolean::<TyInfo>::new(value, span))
    }

    pub fn ident(name: Symbol, span: Span) -> Self {
        Self::Ident(Ident::<TyInfo>::new(name, span))
    }

    pub fn block(statements: Vec<Statement<TyInfo>>, span: Span) -> Self {
        Self::Block(Block::<TyInfo>::new(statements, span))
    }

    pub fn _if(
        condition: Expression<TyInfo>,
        success: Block<TyInfo>,
        otherwise: Option<Block<TyInfo>>,
        span: Span,
    ) -> Self {
        Self::If(If::<TyInfo>::new(
            Box::new(condition),
            success,
            otherwise,
            span,
        ))
    }
}

impl<TyInfo> Spanned for Expression<TyInfo> {
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
