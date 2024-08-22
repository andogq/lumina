use super::*;
use crate::ast_node2;

ast_node2! {
    Block<M> {
        statements: Vec<Statement<M>>,
        span,
        ty_info,
    }
}
