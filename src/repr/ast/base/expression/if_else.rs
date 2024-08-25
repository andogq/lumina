use crate::ast_node;

use super::*;

ast_node! {
    If<M> {
        condition: Box<Expression<M>>,
        success: Block<M>,
        otherwise: Option<Block<M>>,
        span,
        ty_info,
    }
}
