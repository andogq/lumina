use crate::ast_node;

use super::*;

ast_node! {
    struct Program<TyInfo, FnIdentifier, IdentIdentifier> {
        functions: Vec<Function<TyInfo, FnIdentifier, IdentIdentifier>>,
        main: Function<TyInfo, FnIdentifier, IdentIdentifier>,
    }
}
