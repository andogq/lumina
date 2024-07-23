use crate::{ast_node, repr::ty::Ty, util::symbol_map::interner_symbol_map::Symbol};

use super::*;

ast_node! {
    struct Function<TyInfo, FnIdentifier, IdentIdentifier> {
        name: FnIdentifier,
        parameters: Vec<(Symbol, Ty)>,
        return_ty: Ty,
        body: Block<TyInfo, FnIdentifier, IdentIdentifier>,
    }
}
