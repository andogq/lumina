use crate::ast_node;

ast_node! {
    Integer<M> {
        value: i64,
        span,
        ty_info,
    }
}
