use crate::{ast_node, util::symbol_map::SymbolMap};

use super::*;

ast_node! {
    struct Program<TyInfo> {
        functions: Vec<Function<TyInfo>>,
        main: Function<TyInfo>,
        symbols: SymbolMap,
    }
}
