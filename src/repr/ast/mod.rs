use std::fmt::Debug;

mod macros;
pub mod typed;
pub mod untyped;

pub trait AstMetadata {
    type FnIdentifier: Debug + Clone;
    type IdentIdentifier: Debug + Clone;
    type TyInfo: Debug + Clone;
    type Span: Debug + Clone;
}
