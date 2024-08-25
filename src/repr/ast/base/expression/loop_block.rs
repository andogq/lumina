use crate::ast_node;

use super::*;

ast_node! {
    Loop<M> {
        body: Block<M>,
        span,
        ty_info,
    }
}
