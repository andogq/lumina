use crate::{
    core::symbol::Symbol,
    util::source::{Span, Spanned},
};

use super::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Return(ReturnStatement),
    Let(LetStatement),
    Expression(ExpressionStatement),
}

impl Statement {
    pub fn _return(expression: Expression) -> Self {
        Self::Return(ReturnStatement {
            span: Span::default(),
            value: expression,
        })
    }

    pub fn _let(name: Symbol, expression: Expression) -> Self {
        Self::Let(LetStatement {
            span: Span::default(),
            name,
            value: expression,
        })
    }

    pub fn expression(expression: Expression, implicit_return: bool) -> Self {
        Self::Expression(ExpressionStatement {
            span: Span::default(),
            expression,
            implicit_return,
        })
    }
}

impl Spanned for Statement {
    fn span(&self) -> &Span {
        match self {
            Statement::Return(s) => s.span(),
            Statement::Let(s) => s.span(),
            Statement::Expression(s) => s.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub span: Span,
    pub value: Expression,
}

impl Spanned for ReturnStatement {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub span: Span,
    pub name: Symbol,
    pub value: Expression,
}

impl Spanned for LetStatement {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub span: Span,
    pub expression: Expression,
    pub implicit_return: bool,
}

impl Spanned for ExpressionStatement {
    fn span(&self) -> &Span {
        &self.span
    }
}
