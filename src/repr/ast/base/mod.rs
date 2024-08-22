mod expression;
mod function;
mod macros;
mod program;
mod statement;

use std::fmt::Debug;

pub use expression::*;
pub use function::*;
pub use program::*;
pub use statement::*;

pub trait AstMetadata {
    type FnIdentifier: Debug + Clone;
    type IdentIdentifier: Debug + Clone;
    type TyInfo: Debug + Clone;
    type Span: Debug + Clone;
}
