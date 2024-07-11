use crate::{ast_node, util::symbol_map::SymbolMap};

use super::*;

ast_node! {
    struct Program<TyInfo, FnIdentifier> {
        functions: Vec<Function<TyInfo, FnIdentifier>>,
        main: Function<TyInfo, FnIdentifier>,
        symbols: SymbolMap,
    }
}
