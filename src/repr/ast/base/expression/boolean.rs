use crate::ast_node;

ast_node! {
    Boolean<M> {
        value: bool,
        span,
        ty_info,
    }
}
