use super::*;
use crate::ast_node;

ast_node! {
    Assign<M> {
        binding: M::IdentIdentifier,
        value: Box<Expression<M>>,
        span,
        ty_info,
    }
}
