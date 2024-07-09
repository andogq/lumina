use crate::ast_node;

ast_node! {
    struct Integer<TyInfo> {
        value: i64,
    }
}
