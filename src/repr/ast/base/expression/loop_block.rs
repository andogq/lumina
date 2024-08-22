use crate::ast_node2;

use super::*;

ast_node2! {
    Loop<M> {
        body: Block<M>,
        span,
        ty_info,
    }
}
