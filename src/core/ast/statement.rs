use crate::{
    ast_node,
    core::symbol::Symbol,
    util::source::{Span, Spanned},
};

use super::Expression;

#[derive(Debug, Clone)]
pub enum Statement<TyInfo> {
    Return(ReturnStatement<TyInfo>),
    Let(LetStatement<TyInfo>),
    Expression(ExpressionStatement<TyInfo>),
}

impl<TyInfo: Default> Statement<TyInfo> {
    pub fn _return(expression: Expression<TyInfo>, span: Span) -> Self {
        Self::Return(ReturnStatement::new(expression, span))
    }

    pub fn _let(name: Symbol, value: Expression<TyInfo>, span: Span) -> Self {
        Self::Let(LetStatement::new(name, value, span))
    }

    pub fn expression(expression: Expression<TyInfo>, implicit_return: bool, span: Span) -> Self {
        Self::Expression(ExpressionStatement::new(expression, implicit_return, span))
    }
}

impl<TyInfo> Spanned for Statement<TyInfo> {
    fn span(&self) -> &Span {
        match self {
            Statement::Return(s) => s.span(),
            Statement::Let(s) => s.span(),
            Statement::Expression(s) => s.span(),
        }
    }
}

ast_node! {
    struct ReturnStatement<TyInfo> {
        value: Expression<TyInfo>,
    }
}

ast_node! {
    struct LetStatement<TyInfo> {
        name: Symbol,
        value: Expression<TyInfo>,
    }
}

ast_node! {
    struct ExpressionStatement<TyInfo> {
        expression: Expression<TyInfo>,
        implicit_return: bool,
    }
}
