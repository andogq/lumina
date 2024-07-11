use crate::{ast_node, ctx::Symbol, repr::ty::Ty};

use super::*;

ast_node! {
    struct Function<TyInfo, FnIdentifier> {
        name: FnIdentifier,
        parameters: Vec<(Symbol, Ty)>,
        return_ty: Ty,
        body: Block<TyInfo, FnIdentifier>,
    }
}
