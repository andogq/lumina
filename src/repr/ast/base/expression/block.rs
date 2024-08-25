use super::*;
use crate::ast_node;

ast_node! {
    Block<M> {
        statements: Vec<Statement<M>>,
        span,
        ty_info,
    }
}
