use super::*;
use crate::ast_node2;

ast_node2! {
    Assign<M> {
        binding: M::IdentIdentifier,
        value: Box<Expression<M>>,
        span,
        ty_info,
    }
}
