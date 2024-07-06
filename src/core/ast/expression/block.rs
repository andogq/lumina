use crate::{ast_node, core::ast::Statement};

ast_node! {
    struct Block<TyInfo> {
        statements: Vec<Statement<TyInfo>>,
    }
}
