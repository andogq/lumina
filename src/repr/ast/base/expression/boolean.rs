use crate::ast_node;

ast_node! {
    typed struct Boolean<TyInfo> {
        value: bool,
    }
}
