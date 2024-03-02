mod expression;
mod program;
mod statement;

pub use expression::*;
pub use program::*;
pub use statement::*;

use crate::core::lexer::Lexer;

pub trait ParseNode<S>: Sized {
    fn parse(tokens: &mut Lexer<S>) -> Result<Self, String>;
}
