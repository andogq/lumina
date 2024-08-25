use crate::ast_node;

use super::*;

ast_node! {
    Call<M> {
        name: M::FnIdentifier,
        args: Vec<Expression<M>>,
        span,
        ty_info,
    }
}
