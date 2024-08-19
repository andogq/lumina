use crate::ast_node;

use super::*;

ast_node! {
    typed struct Loop<TyInfo, FnIdentifier, IdentIdentifier> {
        body: Block<TyInfo, FnIdentifier, IdentIdentifier>,
    }
}
