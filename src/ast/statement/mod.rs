mod s_let;
mod s_return;

pub use s_let::*;
pub use s_return::*;

use crate::ast::Expression;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(Expression),
}
