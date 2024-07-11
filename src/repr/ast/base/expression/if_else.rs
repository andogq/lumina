use crate::ast_node;

use super::*;

ast_node! {
    typed struct If<TyInfo, FnIdentifier> {
        condition: Box<Expression<TyInfo, FnIdentifier>>,
        success: Block<TyInfo, FnIdentifier>,
        otherwise: Option<Block<TyInfo, FnIdentifier>>,
    }
}
