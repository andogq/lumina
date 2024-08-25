use crate::{ast_node, repr::ty::Ty};

use super::Expression;

ast_node! {
    Cast<M> {
        value: Box<Expression<M>>,
        target_ty: Ty,
        span,
        ty_info,
    }
}
