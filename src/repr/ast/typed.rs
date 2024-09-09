use crate::{
    generate_ast,
    repr::identifier::{FunctionIdx, ScopedBinding},
    ty::TyInfo,
    util::span::Span,
};

use super::AstMetadata;

pub struct TypedAstMetadata;
impl AstMetadata for TypedAstMetadata {
    type FnIdentifier = FunctionIdx;
    type IdentIdentifier = ScopedBinding;
    type TyInfo = TyInfo;
    type Span = Span;
}

generate_ast!(TypedAstMetadata);
