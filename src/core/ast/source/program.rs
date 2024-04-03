use crate::core::ast::symbol::SymbolMap;

use super::function::Function;

pub struct Program {
    pub functions: Vec<Function>,
    pub main: Function,
    pub symbol_map: SymbolMap,
}
