use std::collections::HashMap;

use index_vec::IndexVec;

use crate::{
    repr::{
        ast::typed::Function,
        identifier::{FunctionIdx, ScopedBinding},
        ir,
        ty::Ty,
    },
    stage::{
        lower_ir::{FunctionBuilder as FunctionBuilderTrait, IRCtx},
        parse::ParseCtx,
        type_check::{FunctionSignature, TypeCheckCtx},
    },
    util::symbol_map::{interner_symbol_map::*, SymbolMap},
};

pub struct CompilePass {
    symbols: InternerSymbolMap,

    function_signatures: IndexVec<FunctionIdx, FunctionSignature>,
    function_symbols: HashMap<Symbol, FunctionIdx>,

    ir_functions: HashMap<FunctionIdx, ir::Function>,
}

impl CompilePass {
    pub fn new() -> Self {
        Self {
            symbols: InternerSymbolMap::new(),
            function_signatures: IndexVec::new(),
            function_symbols: HashMap::new(),
            ir_functions: HashMap::new(),
        }
    }
}

impl Default for CompilePass {
    fn default() -> Self {
        Self::new()
    }
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

impl IRCtx for CompilePass {
    type FunctionBuilder = FunctionBuilder;

    fn register_function(&mut self, idx: FunctionIdx, function: ir::Function) {
        self.ir_functions.insert(idx, function);
    }

    fn all_functions(&self) -> Vec<(FunctionIdx, ir::Function)> {
        self.ir_functions
            .iter()
            .map(|(idx, function)| (*idx, function.clone()))
            .collect()
    }
}

pub struct FunctionBuilder {
    idx: FunctionIdx,
    signature: FunctionSignature,

    basic_blocks: IndexVec<ir::BasicBlockIdx, ir::BasicBlock>,
    current_basic_block: ir::BasicBlockIdx,

    scope: Vec<(ScopedBinding, Ty)>,
}

impl FunctionBuilderTrait for FunctionBuilder {
    fn new(function: &Function) -> Self {
        let mut basic_blocks = IndexVec::new();
        let current_basic_block = basic_blocks.push(ir::BasicBlock::default());

        Self {
            idx: function.name,
            signature: FunctionSignature::from(function),
            basic_blocks,
            current_basic_block,
            scope: function.parameters.to_vec(),
        }
    }

    fn register_scoped(&mut self, ident: ScopedBinding, ty: Ty) {
        self.scope.push((ident, ty));
    }

    fn add_triple(&mut self, triple: ir::Triple) -> ir::TripleRef {
        ir::TripleRef {
            basic_block: self.current_basic_block,
            triple: self.basic_blocks[self.current_basic_block]
                .triples
                .push(triple),
        }
    }

    fn current_bb(&self) -> ir::BasicBlockIdx {
        self.current_basic_block
    }

    fn goto_bb(&mut self, bb: ir::BasicBlockIdx) {
        assert!(
            bb < self.basic_blocks.len_idx(),
            "can only goto basic block if it exists"
        );
        self.current_basic_block = bb;
    }

    fn push_bb(&mut self) -> ir::BasicBlockIdx {
        let idx = self.basic_blocks.push(ir::BasicBlock::default());

        self.current_basic_block = idx;

        idx
    }

    fn build<Ctx: IRCtx<FunctionBuilder = Self>>(self, ctx: &mut Ctx) {
        ctx.register_function(
            self.idx,
            ir::Function {
                symbol: self.idx,
                signature: self.signature,
                basic_blocks: self.basic_blocks,
                scope: self.scope.into_iter().map(|(symbol, _)| symbol).collect(),
            },
        )
    }
}
