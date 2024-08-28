use crate::{
    generate_ast,
    repr::{
        identifier::{FunctionIdx, ScopedBinding},
        ty::Ty,
    },
    util::span::Span,
};

use super::AstMetadata;

#[derive(Clone, Debug)]
pub struct TyInfo {
    pub ty: Ty,
    pub return_ty: Option<Ty>,
}

pub struct TypedAstMetadata;
impl AstMetadata for TypedAstMetadata {
    type FnIdentifier = FunctionIdx;
    type IdentIdentifier = ScopedBinding;
    type TyInfo = TyInfo;
    type Span = Span;
}

generate_ast!(TypedAstMetadata);
