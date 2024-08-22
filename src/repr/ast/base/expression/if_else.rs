use crate::ast_node2;

use super::*;

ast_node2! {
    If<M> {
        condition: Box<Expression<M>>,
        success: Block<M>,
        otherwise: Option<Block<M>>,
        span,
        ty_info,
    }
}
