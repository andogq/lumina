use super::*;
use crate::ast_node;

ast_node! {
    typed struct Assign<TyInfo, FnIdentifier, IdentIdentifier> {
        binding: IdentIdentifier,
        value: Box<Expression<TyInfo, FnIdentifier, IdentIdentifier>>,
    }
}
