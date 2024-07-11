use crate::{ast_node, ctx::Symbol};

use super::*;

ast_node! {
    typed struct Call<TyInfo> {
        name: Symbol,
        args: Vec<Expression<TyInfo>>,
    }
}
