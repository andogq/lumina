use crate::{
    ast_node,
    core::{ctx::Symbol, ty::Ty},
};

use super::Block;

ast_node! {
    struct Function<TyInfo> {
        name: Symbol,
        parameters: Vec<(Symbol, Ty)>,
        return_ty: Ty,
        body: Block<TyInfo>,
    }
}
