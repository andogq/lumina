mod block;
mod boolean;
mod call;
mod ident;
mod if_else;
mod infix;
mod integer;

pub use block::*;
pub use boolean::*;
pub use call::*;
pub use ident::*;
pub use if_else::*;
pub use infix::*;
pub use integer::*;

use crate::{
    ast_node,
    ctx::Symbol,
    util::source::{Span, Spanned},
};

use super::Statement;

ast_node!(
    enum Expression<TyInfo, FnIdentifier> {
        Infix(Infix<TyInfo, FnIdentifier>),
        Integer(Integer<TyInfo>),
        Boolean(Boolean<TyInfo>),
        Ident(Ident<TyInfo>),
        Block(Block<TyInfo, FnIdentifier>),
        If(If<TyInfo, FnIdentifier>),
        Call(Call<TyInfo, FnIdentifier>),
    }
);

impl<TyInfo: Default, FnIdentifier> Expression<TyInfo, FnIdentifier> {
    pub fn infix(
        left: Expression<TyInfo, FnIdentifier>,
        operation: InfixOperation,
        right: Expression<TyInfo, FnIdentifier>,
    ) -> Self {
        let span = Spanned::span(&left).to(&right);
        Self::Infix(Infix::<TyInfo, FnIdentifier>::new(
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

    pub fn block(statements: Vec<Statement<TyInfo, FnIdentifier>>, span: Span) -> Self {
        Self::Block(Block::<TyInfo, FnIdentifier>::new(statements, span))
    }

    pub fn _if(
        condition: Expression<TyInfo, FnIdentifier>,
        success: Block<TyInfo, FnIdentifier>,
        otherwise: Option<Block<TyInfo, FnIdentifier>>,
        span: Span,
    ) -> Self {
        Self::If(If::<TyInfo, FnIdentifier>::new(
            Box::new(condition),
            success,
            otherwise,
            span,
        ))
    }

    pub fn call(
        identifier: FnIdentifier,
        args: Vec<Expression<TyInfo, FnIdentifier>>,
        span: Span,
    ) -> Self {
        Self::Call(Call::new(identifier, args, span))
    }
}
