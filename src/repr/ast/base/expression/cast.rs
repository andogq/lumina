use crate::{ast_node2, repr::ty::Ty};

use super::Expression;

ast_node2! {
    Cast<M> {
        value: Box<Expression<M>>,
        target_ty: Ty,
        span,
        ty_info,
    }
}
