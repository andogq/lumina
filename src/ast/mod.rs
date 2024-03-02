mod expression;
mod program;
mod statement;

use std::fmt::Display;

pub use expression::*;
pub use program::*;
pub use statement::*;

use crate::{
    interpreter::{environment::Environment, return_value::Return},
    lexer::Lexer,
    object::Object,
};

pub trait AstNode: Display + Sized {
    fn evaluate(&self, env: Environment) -> Return<Object>;
}

pub trait ParseNode<S>: AstNode {
    fn parse(tokens: &mut Lexer<S>) -> Result<Self, String>;
}
