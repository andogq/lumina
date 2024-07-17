use std::collections::HashMap;

use index_vec::IndexVec;

use crate::{
    repr::identifier::FunctionIdx,
    stage::{
        parse::ParseCtx,
        type_check::{FunctionSignature, TypeCheckCtx},
    },
    util::symbol_map::{interner_symbol_map::*, SymbolMap},
};

#[derive(Default)]
pub struct CompilePass {
    symbols: InternerSymbolMap,

    function_signatures: IndexVec<FunctionIdx, FunctionSignature>,
    function_symbols: HashMap<Symbol, FunctionIdx>,
}

impl SymbolMap for CompilePass {
    type Symbol = Symbol;

    fn intern<T>(&mut self, s: T) -> Symbol
    where
        T: AsRef<str>,
    {
        SymbolMap::intern(&mut self.symbols, s)
    }

    fn get(&self, s: Symbol) -> String {
        SymbolMap::get(&self.symbols, s)
    }

    fn dump_symbols(&self) -> InternerSymbolMap {
        SymbolMap::dump_symbols(&self.symbols)
    }
}

impl ParseCtx for CompilePass {}

impl TypeCheckCtx for CompilePass {
    fn register_function(&mut self, symbol: Symbol, signature: FunctionSignature) -> FunctionIdx {
        let idx = self.function_signatures.push(signature);
        self.function_symbols.insert(symbol, idx);
        idx
    }

    fn get_function(&self, idx: FunctionIdx) -> FunctionSignature {
        self.function_signatures[idx].clone()
    }

    fn lookup_function_symbol(&self, symbol: Symbol) -> Option<FunctionIdx> {
        self.function_symbols.get(&symbol).cloned()
    }
}
