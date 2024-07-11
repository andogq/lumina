use crate::ast_node;

ast_node! {
    typed struct Integer<TyInfo> {
        value: i64,
    }
}
