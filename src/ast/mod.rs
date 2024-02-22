mod expression;
mod program;
mod statement;

use std::{fmt::Display, iter::Peekable};

pub use expression::*;
pub use program::*;
pub use statement::*;

use crate::{object::Object, token::Token};

pub trait AstNode: Display + Sized {
    fn evaluate(&self) -> Object;
}

pub trait ParseNode: AstNode {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String>;
}
