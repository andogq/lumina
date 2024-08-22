use crate::ast_node2;

use super::*;

ast_node2! {
    Call<M> {
        name: M::FnIdentifier,
        args: Vec<Expression<M>>,
        span,
        ty_info,
    }
}
