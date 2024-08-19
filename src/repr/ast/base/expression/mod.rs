mod block;
mod boolean;
mod call;
mod ident;
mod if_else;
mod infix;
mod integer;
mod loop_block;

pub use block::*;
pub use boolean::*;
pub use call::*;
pub use ident::*;
pub use if_else::*;
pub use infix::*;
pub use integer::*;
pub use loop_block::*;

use crate::{ast_node, util::span::Span};

use super::Statement;

ast_node!(
    enum Expression<TyInfo, FnIdentifier, IdentIdentifier> {
        Infix(Infix<TyInfo, FnIdentifier, IdentIdentifier>),
        Integer(Integer<TyInfo>),
        Boolean(Boolean<TyInfo>),
        Ident(Ident<TyInfo, IdentIdentifier>),
        Block(Block<TyInfo, FnIdentifier, IdentIdentifier>),
        If(If<TyInfo, FnIdentifier, IdentIdentifier>),
        Call(Call<TyInfo, FnIdentifier, IdentIdentifier>),
        Loop(Loop<TyInfo, FnIdentifier, IdentIdentifier>),
    }
);

impl<TyInfo: Default, FnIdentifier, IdentIdentifier>
    Expression<TyInfo, FnIdentifier, IdentIdentifier>
{
    pub fn infix(
        left: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
        operation: InfixOperation,
        right: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
    ) -> Self {
        let span = left.span().start..right.span().end;
        Self::Infix(Infix::<TyInfo, FnIdentifier, IdentIdentifier>::new(
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

    pub fn ident(name: IdentIdentifier, span: Span) -> Self {
        Self::Ident(Ident::<TyInfo, IdentIdentifier>::new(name, span))
    }

    pub fn block(
        statements: Vec<Statement<TyInfo, FnIdentifier, IdentIdentifier>>,
        span: Span,
    ) -> Self {
        Self::Block(Block::<TyInfo, FnIdentifier, IdentIdentifier>::new(
            statements, span,
        ))
    }

    pub fn _if(
        condition: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
        success: Block<TyInfo, FnIdentifier, IdentIdentifier>,
        otherwise: Option<Block<TyInfo, FnIdentifier, IdentIdentifier>>,
        span: Span,
    ) -> Self {
        Self::If(If::<TyInfo, FnIdentifier, IdentIdentifier>::new(
            Box::new(condition),
            success,
            otherwise,
            span,
        ))
    }

    pub fn call(
        identifier: FnIdentifier,
        args: Vec<Expression<TyInfo, FnIdentifier, IdentIdentifier>>,
        span: Span,
    ) -> Self {
        Self::Call(Call::new(identifier, args, span))
    }
}
