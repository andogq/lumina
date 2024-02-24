mod expression;
mod program;
mod statement;

use std::{fmt::Display, iter::Peekable};

pub use expression::*;
pub use program::*;
pub use statement::*;

use crate::{
    interpreter::{environment::Environment, object::Object, return_value::Return},
    token::Token,
};

pub trait AstNode: Display + Sized {
    fn evaluate(&self, env: Environment) -> Return<Object>;
}

pub trait ParseNode: AstNode {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String>;
}
