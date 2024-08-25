use super::*;
use crate::ast_node;

ast_node! {
    Array<M> {
        init: Vec<Expression<M>>,
        span,
        ty_info,
    }
}
