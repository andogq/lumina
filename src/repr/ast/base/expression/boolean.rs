use crate::ast_node;

ast_node! {
    struct Boolean<TyInfo> {
        value: bool,
    }
}
