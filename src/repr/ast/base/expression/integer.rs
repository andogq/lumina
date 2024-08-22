use crate::ast_node2;

ast_node2! {
    Integer<M> {
        value: i64,
        span,
        ty_info,
    }
}
