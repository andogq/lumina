use crate::{ast_node, repr::ty::Ty};

use super::*;

ast_node! {
    struct Function<TyInfo, FnIdentifier, IdentIdentifier> {
        name: FnIdentifier,
        parameters: Vec<(IdentIdentifier, Ty)>,
        return_ty: Ty,
        body: Block<TyInfo, FnIdentifier, IdentIdentifier>,
    }
}
