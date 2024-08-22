use crate::ast_node2;

ast_node2! {
    Boolean<M> {
        value: bool,
        span,
        ty_info,
    }
}
