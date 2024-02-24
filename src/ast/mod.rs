mod expression;
mod program;
mod statement;

use std::{
    fmt::{Display, Formatter, Pointer},
    iter::Peekable,
};

pub use expression::*;
pub use program::*;
pub use statement::*;

use crate::{object::Object, token::Token};

pub enum Return<T> {
    Explicit(T),
    Implicit(T),
    Error(T),
}

impl<T> Return<T> {
    pub fn value(self) -> T {
        match self {
            Return::Explicit(value) => value,
            Return::Implicit(value) => value,
            Return::Error(value) => value,
        }
    }
}

impl<T> Display for Return<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Return::Explicit(value) | Return::Implicit(value) | Return::Error(value) => {
                value.fmt(f)
            }
        }
    }
}

pub trait AstNode: Display + Sized {
    fn evaluate(&self) -> Return<Object>;
}

pub trait ParseNode: AstNode {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Self, String>;
}
