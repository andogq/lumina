use crate::ast_node;

use super::*;

ast_node! {
    typed struct If<TyInfo, FnIdentifier, IdentIdentifier> {
        condition: Box<Expression<TyInfo, FnIdentifier, IdentIdentifier>>,
        success: Block<TyInfo, FnIdentifier, IdentIdentifier>,
        otherwise: Option<Block<TyInfo, FnIdentifier, IdentIdentifier>>,
    }
}
