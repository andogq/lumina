mod array;
mod assign;
mod block;
mod boolean;
mod call;
mod cast;
mod ident;
mod if_else;
mod index;
mod infix;
mod integer;
mod loop_block;

pub use array::*;
pub use assign::*;
pub use block::*;
pub use boolean::*;
pub use call::*;
pub use cast::*;
pub use ident::*;
pub use if_else::*;
pub use index::*;
pub use infix::*;
pub use integer::*;
pub use loop_block::*;

use crate::{ast_node, util::span::Span};

use super::{AstMetadata, Statement};

ast_node! {
    Expression<M>(
        Array,
        Infix,
        Integer,
        Boolean,
        Ident,
        Block,
        If,
        Index,
        Call,
        Loop,
        Assign,
        Cast,
    )
}

impl<M: AstMetadata<Span = Span, TyInfo: Default>> Expression<M> {
    pub fn infix(left: Expression<M>, operation: InfixOperation, right: Expression<M>) -> Self {
        let span = left.span().start..right.span().end;
        Self::Infix(Infix::<M>::new(
            Box::new(left),
            operation,
            Box::new(right),
            span,
            M::TyInfo::default(),
        ))
    }

    pub fn integer(value: i64, span: Span) -> Self {
        Self::Integer(Integer::new(value, span, M::TyInfo::default()))
    }

    pub fn boolean(value: bool, span: Span) -> Self {
        Self::Boolean(Boolean::new(value, span, M::TyInfo::default()))
    }

    pub fn ident(name: M::IdentIdentifier, span: Span) -> Self {
        Self::Ident(Ident::new(name, span, M::TyInfo::default()))
    }

    pub fn block(statements: Vec<Statement<M>>, span: Span) -> Self {
        Self::Block(Block::new(statements, span, M::TyInfo::default()))
    }

    pub fn _if(
        condition: Expression<M>,
        success: Block<M>,
        otherwise: Option<Block<M>>,
        span: Span,
    ) -> Self {
        Self::If(If::new(
            Box::new(condition),
            success,
            otherwise,
            span,
            M::TyInfo::default(),
        ))
    }

    pub fn call(identifier: M::FnIdentifier, args: Vec<Expression<M>>, span: Span) -> Self {
        Self::Call(Call::new(identifier, args, span, M::TyInfo::default()))
    }
}
