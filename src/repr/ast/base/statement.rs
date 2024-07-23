use crate::{ast_node, util::source::Span};

use super::*;

ast_node!(
    enum Statement<TyInfo, FnIdentifier, IdentIdentifier> {
        Return(ReturnStatement<TyInfo, FnIdentifier, IdentIdentifier>),
        Let(LetStatement<TyInfo, FnIdentifier, IdentIdentifier>),
        Expression(ExpressionStatement<TyInfo, FnIdentifier, IdentIdentifier>),
    }
);

impl<TyInfo: Default, FnIdentifier, IdentIdentifier>
    Statement<TyInfo, FnIdentifier, IdentIdentifier>
{
    pub fn _return(
        expression: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
        span: Span,
    ) -> Self {
        Self::Return(ReturnStatement::new(expression, span))
    }

    pub fn _let(
        name: IdentIdentifier,
        value: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
        span: Span,
    ) -> Self {
        Self::Let(LetStatement::new(name, value, span))
    }

    pub fn expression(
        expression: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
        implicit_return: bool,
        span: Span,
    ) -> Self {
        Self::Expression(ExpressionStatement::new(expression, implicit_return, span))
    }
}

ast_node! {
    typed struct ReturnStatement<TyInfo, FnIdentifier, IdentIdentifier> {
        value: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
    }
}

ast_node! {
    typed struct LetStatement<TyInfo, FnIdentifier, IdentIdentifier> {
        binding: IdentIdentifier,
        value: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
    }
}

ast_node! {
    typed struct ExpressionStatement<TyInfo, FnIdentifier, IdentIdentifier> {
        expression: Expression<TyInfo, FnIdentifier, IdentIdentifier>,
        implicit_return: bool,
    }
}
