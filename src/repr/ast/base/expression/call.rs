use crate::ast_node;

use super::*;

ast_node! {
    typed struct Call<TyInfo, FnIdentifier> {
        name: FnIdentifier,
        args: Vec<Expression<TyInfo, FnIdentifier>>,
    }
}
