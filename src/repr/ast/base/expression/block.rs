use super::*;
use crate::ast_node;

ast_node! {
    struct Block<TyInfo> {
        statements: Vec<Statement<TyInfo>>,
    }
}
