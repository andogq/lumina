use crate::{ast_node, util::symbol_map::interner_symbol_map::InternerSymbolMap};

use super::*;

ast_node! {
    struct Program<TyInfo, FnIdentifier, IdentIdentifier> {
        functions: Vec<Function<TyInfo, FnIdentifier, IdentIdentifier>>,
        main: Function<TyInfo, FnIdentifier, IdentIdentifier>,
        symbols: InternerSymbolMap,
    }
}
