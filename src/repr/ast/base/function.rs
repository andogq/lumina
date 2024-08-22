use crate::{ast_node2, repr::ty::Ty};

use super::*;

ast_node2! {
    Function<M> {
        name: M::FnIdentifier,
        parameters: Vec<(M::IdentIdentifier, Ty)>,
        return_ty: Ty,
        body: Block<M>,
        span,
    }
}
