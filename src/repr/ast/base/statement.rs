use crate::ast_node2;

use super::*;

ast_node2! {
    Statement<M>(
        Return,
        Let,
        ExpressionStatement,
        Break,
        Continue,
    )
}

impl<M: AstMetadata<TyInfo: Default>> Statement<M> {
    pub fn _return(expression: Expression<M>, span: M::Span) -> Self {
        Self::Return(Return::new(expression, span, M::TyInfo::default()))
    }

    pub fn _let(name: M::IdentIdentifier, value: Expression<M>, span: M::Span) -> Self {
        Self::Let(Let::new(name, value, span, M::TyInfo::default()))
    }

    pub fn expression(expression: Expression<M>, implicit_return: bool, span: M::Span) -> Self {
        Self::ExpressionStatement(ExpressionStatement::new(
            expression,
            implicit_return,
            span,
            M::TyInfo::default(),
        ))
    }
}

ast_node2! {
    Return<M> {
        value: Expression<M>,
        span,
        ty_info,
    }
}

ast_node2! {
    Let<M> {
        binding: M::IdentIdentifier,
        value: Expression<M>,
        span,
        ty_info,
    }
}

ast_node2! {
    ExpressionStatement<M> {
        expression: Expression<M>,
        implicit_return: bool,
        span,
        ty_info,
    }
}

ast_node2! {
    Break<M> {
        span,
        ty_info,
    }
}

ast_node2! {
    Continue<TyInfo> {
        span,
        ty_info,
    }
}
