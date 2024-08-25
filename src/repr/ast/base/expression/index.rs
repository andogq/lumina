use super::*;
use crate::ast_node;

ast_node! {
    Index<M> {
        value: M::IdentIdentifier,
        index: Box<Expression<M>>,
        span,
        ty_info,
    }
}
