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
    fn compile(&self, register_constant: impl FnMut(Object) -> u32) -> Result<Vec<u8>, String>;
}

pub trait ParseNode<S>: AstNode {
    fn parse(tokens: &mut Lexer<S>) -> Result<Self, String>;
}
