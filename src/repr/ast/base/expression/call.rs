use crate::{ast_node, ctx::Symbol};

use super::*;

ast_node! {
    struct Call<TyInfo> {
        name: Symbol,
        args: Vec<Expression<TyInfo>>,
    }
}
