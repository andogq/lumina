use crate::ast_node;

use super::*;

ast_node! {
    typed struct If<TyInfo> {
        condition: Box<Expression<TyInfo>>,
        success: Block<TyInfo>,
        otherwise: Option<Block<TyInfo>>,
    }
}
