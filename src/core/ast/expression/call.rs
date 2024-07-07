use crate::{ast_node, core::symbol::Symbol};

use super::Expression;

ast_node! {
    struct Call<TyInfo> {
        name: Symbol,
        args: Vec<Expression<TyInfo>>,
    }
}
