use crate::{ast_node, core::ctx::SymbolMap};

use super::*;

ast_node! {
    struct Program<TyInfo> {
        functions: Vec<Function<TyInfo>>,
        main: Function<TyInfo>,
        symbols: SymbolMap,
    }
}
