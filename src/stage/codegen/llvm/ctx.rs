use std::collections::HashMap;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    types::BasicType,
    values::{FunctionValue, IntValue, PointerValue},
};

use crate::{
    repr::{
        identifier::FunctionIdx,
        ir::{self, BasicBlockIdx, Triple},
        ty::Ty,
    },
    util::symbol_map::{interner_symbol_map::Symbol, SymbolMap},
};

pub trait LLVMCodegenCtx<'ctx>: SymbolMap<Symbol = Symbol> {
    type FunctionBuilder: LLVMFunctionBuilder<'ctx>;

    /// Create a new LLVM builder.
    fn new_builder(&self) -> Builder<'ctx>;

    /// Lookup the LLVM function value associated with an identifier.
    fn lookup_function_value(&self, function_idx: FunctionIdx) -> Option<FunctionValue<'ctx>>;

    /// Create the LLVM function value for the given identifier, and registers it. Assumes that
    /// the value didn't already exist.
    fn create_function_value(&mut self, function_idx: FunctionIdx) -> FunctionValue<'ctx>;

    /// Append a basic block to the provided function.
    fn append_basic_block(&mut self, function_idx: FunctionIdx) -> BasicBlock<'ctx>;

    /// Get the IR version of a function.
    fn get_function(&self, function_idx: FunctionIdx) -> ir::Function;

    /// Produce the LLVM basic type that corresponds with a [`Ty`]
    fn get_type(&self, ty: Ty) -> impl BasicType<'ctx>;

    fn const_int(&self, value: i64) -> IntValue<'ctx>;
    fn const_bool(&self, value: bool) -> IntValue<'ctx>;

    fn create_function_builder(
        &self,
        symbol_locations: HashMap<Symbol, PointerValue<'ctx>>,
    ) -> Self::FunctionBuilder;
}

pub trait LLVMFunctionBuilder<'ctx> {
    fn lookup_basic_block(&self, basic_block_idx: BasicBlockIdx) -> Option<BasicBlock<'ctx>>;
    fn create_basic_block(&mut self, basic_block_idx: BasicBlockIdx) -> BasicBlock<'ctx>;

    /// Create a basic block that is not associated with an IR basic block.
    fn anonymous_basic_block(&self) -> BasicBlock<'ctx>;

    fn lookup_symbol(&self, symbol: Symbol) -> PointerValue<'ctx>;

    fn get_triples(&self, basic_block_idx: BasicBlockIdx) -> Vec<Triple>;
}
