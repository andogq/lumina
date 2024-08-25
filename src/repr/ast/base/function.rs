use crate::{ast_node, repr::ty::Ty};

use super::*;

ast_node! {
    Function<M> {
        name: M::FnIdentifier,
        parameters: Vec<(M::IdentIdentifier, Ty)>,
        return_ty: Ty,
        body: Block<M>,
        span,
    }
}
