use crate::{ast_node, ctx::Symbol, util::source::Span};

use super::*;

ast_node!(
    enum Statement<TyInfo, FnIdentifier> {
        Return(ReturnStatement<TyInfo, FnIdentifier>),
        Let(LetStatement<TyInfo, FnIdentifier>),
        Expression(ExpressionStatement<TyInfo, FnIdentifier>),
    }
);

impl<TyInfo: Default, FnIdentifier> Statement<TyInfo, FnIdentifier> {
    pub fn _return(expression: Expression<TyInfo, FnIdentifier>, span: Span) -> Self {
        Self::Return(ReturnStatement::new(expression, span))
    }

    pub fn _let(name: Symbol, value: Expression<TyInfo, FnIdentifier>, span: Span) -> Self {
        Self::Let(LetStatement::new(name, value, span))
    }

    pub fn expression(
        expression: Expression<TyInfo, FnIdentifier>,
        implicit_return: bool,
        span: Span,
    ) -> Self {
        Self::Expression(ExpressionStatement::new(expression, implicit_return, span))
    }
}

ast_node! {
    typed struct ReturnStatement<TyInfo, FnIdentifier> {
        value: Expression<TyInfo, FnIdentifier>,
    }
}

ast_node! {
    typed struct LetStatement<TyInfo, FnIdentifier> {
        name: Symbol,
        value: Expression<TyInfo, FnIdentifier>,
    }
}

ast_node! {
    typed struct ExpressionStatement<TyInfo, FnIdentifier> {
        expression: Expression<TyInfo, FnIdentifier>,
        implicit_return: bool,
    }
}
