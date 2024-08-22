use crate::{
    generate_ast,
    repr::{
        identifier::{FunctionIdx, ScopedBinding},
        ty::Ty,
    },
    util::span::Span,
};

use super::base::AstMetadata;

#[derive(Clone, Debug)]
pub struct TyInfo {
    pub ty: Ty,
    pub return_ty: Option<Ty>,
}

pub struct TypedAstMetdata;
impl AstMetadata for TypedAstMetdata {
    type FnIdentifier = FunctionIdx;
    type IdentIdentifier = ScopedBinding;
    type TyInfo = TyInfo;
    type Span = Span;
}

generate_ast!(TypedAstMetdata);
