use super::*;
use crate::ast_node;

ast_node! {
    typed struct Block<TyInfo, FnIdentifier> {
        statements: Vec<Statement<TyInfo, FnIdentifier>>,
    }
}
