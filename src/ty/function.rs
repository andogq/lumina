use crate::{hir, repr::ast::AstMetadata};

use super::*;

#[derive(Clone, Debug)]
pub struct FunctionSignature {
    pub arguments: Vec<Ty>,
    pub return_ty: Ty,
}

impl<M: AstMetadata> From<&hir::Function<M>> for FunctionSignature {
    fn from(function: &hir::Function<M>) -> Self {
        Self {
            arguments: function
                .parameters
                .iter()
                .map(|(_, ty)| ty.clone())
                .collect(),
            return_ty: function.return_ty.clone(),
        }
    }
}
