use crate::ast_node;

use super::*;

ast_node! {
    typed struct Call<TyInfo, FnIdentifier, IdentIdentifier> {
        name: FnIdentifier,
        args: Vec<Expression<TyInfo, FnIdentifier, IdentIdentifier>>,
    }
}
