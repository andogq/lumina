use crate::ast_node;

use super::{Block, Expression};

ast_node! {
    struct If<TyInfo> {
        condition: Box<Expression<TyInfo>>,
        success: Block<TyInfo>,
        otherwise: Option<Block<TyInfo>>,
    }
}
