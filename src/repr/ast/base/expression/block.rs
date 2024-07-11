use super::*;
use crate::ast_node;

ast_node! {
    typed struct Block<TyInfo> {
        statements: Vec<Statement<TyInfo>>,
    }
}
