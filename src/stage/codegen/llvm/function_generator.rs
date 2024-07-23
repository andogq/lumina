use std::collections::HashMap;

use index_vec::IndexVec;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::BasicType,
    values::{BasicValue, FunctionValue, IntValue, PointerValue},
    IntPredicate,
};

use crate::{
    repr::{
        identifier::FunctionIdx,
        ir::{BasicBlockIdx, BinaryOp, ConstantValue, Function, Triple, TripleIdx, UnaryOp, Value},
        ty::Ty,
    },
    util::symbol_map::{interner_symbol_map::Symbol, SymbolMap},
};

pub struct FunctionGenerator<'ctx, 'ink, Ctx> {
    ctx: &'ctx mut Ctx,
    llvm_ctx: &'ink Context,

    /// IR for this function
    function: Function,
    /// LLVM function value assigned for this function
    llvm_function: FunctionValue<'ink>,
    builder: Builder<'ink>,

    /// All available functions to call
    functions: HashMap<FunctionIdx, FunctionValue<'ink>>,

    /// Resulting values for each of the triples
    // TODO: Replace this with a more robust system
    results: IndexVec<TripleIdx, Option<IntValue<'ink>>>,
    symbols: HashMap<Symbol, PointerValue<'ink>>,

    blocks: HashMap<BasicBlockIdx, BasicBlock<'ink>>,
}

impl<'ctx, 'ink, Ctx: FunctionGeneratorCtx> FunctionGenerator<'ctx, 'ink, Ctx> {
    pub fn new(
        ctx: &'ctx mut Ctx,
        llvm_ctx: &'ink Context,
        functions: HashMap<FunctionIdx, FunctionValue<'ink>>,
        // NOTE: Pre-generate function value, so that a function map can be supplied even if other functions aren't generated
        llvm_function: FunctionValue<'ink>,
        function: Function,
    ) -> Self {
        let builder = llvm_ctx.create_builder();

        // Set up the entry block for this function
        assert_eq!(
            llvm_function.count_basic_blocks(),
            0,
            "function should not have any basic blocks"
        );
        llvm_ctx.append_basic_block(llvm_function, "entry");

        Self {
            ctx,
            llvm_ctx,
            llvm_function,
            functions,
            builder,
            function,
            symbols: HashMap::new(),
            results: IndexVec::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn codegen(&mut self) {
        // Set up all the parameters
        self.function
            .scope
            .iter()
            .map(|symbol| {
                (
                    *symbol,
                    self.alloca(
                        // TODO: Actually determine the type of this symbol
                        Ty::Int,
                        &self.ctx.get(*symbol),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
    }

    fn gen_block(&mut self, block: &BasicBlockIdx) -> BasicBlock<'ink> {
        if let Some(block) = self.blocks.get(block) {
            return *block;
        }

        let basic_block = self
            .llvm_ctx
            .append_basic_block(self.llvm_function, &format!("bb_{block:?}"));
        self.blocks.insert(*block, basic_block.clone());

        let block = self.function.basic_blocks.get(*block).unwrap().clone();

        for triple in &block.triples {
            let result = match triple {
                Triple::BinaryOp { lhs, rhs, op } => Some(self.gen_op_binary(lhs, rhs, op)),
                Triple::UnaryOp { rhs, op } => Some(self.gen_op_unary(rhs, op)),
                Triple::Copy(value) => Some(self.gen_copy(value)),
                Triple::Jump(bb) => {
                    self.gen_jump(bb);
                    None
                }
                Triple::Call(function) => Some(self.gen_call(function)),
                Triple::Return(value) => {
                    self.gen_return(value);
                    None
                }
                Triple::Assign(symbol, value) => {
                    self.gen_assign(symbol, value);
                    None
                }
                Triple::Switch {
                    value,
                    default,
                    branches,
                } => todo!(),
            };
        }

        basic_block
    }

    fn gen_op_binary(&mut self, lhs: &Value, rhs: &Value, op: &BinaryOp) -> IntValue<'ink> {
        let lhs = self
            .retrieve_value(lhs)
            .expect("lhs of binary op cannot be unit");
        let rhs = self
            .retrieve_value(rhs)
            .expect("rhs of binary cannot be unit");

        match op {
            BinaryOp::Add => self.builder.build_int_add(lhs, rhs, "add").unwrap(),
            BinaryOp::Sub => self.builder.build_int_sub(lhs, rhs, "sub").unwrap(),
            BinaryOp::Eq => self
                .builder
                .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq")
                .unwrap(),
            BinaryOp::NotEq => self
                .builder
                .build_int_compare(IntPredicate::NE, lhs, rhs, "not eq")
                .unwrap(),
        }
    }

    fn gen_op_unary(&mut self, rhs: &Value, op: &UnaryOp) -> IntValue<'ink> {
        let rhs = self
            .retrieve_value(rhs)
            .expect("rhs of unary cannot be unit");

        match op {
            UnaryOp::Minus => self.builder.build_int_neg(rhs, "neg").unwrap(),
            UnaryOp::Not => self.builder.build_not(rhs, "not").unwrap(),
        }
    }

    fn gen_copy(&mut self, value: &Value) -> IntValue<'ink> {
        self.retrieve_value(value).unwrap()
    }

    fn gen_jump(&mut self, bb: &BasicBlockIdx) {
        // Ensure the basic block is compiled
        let bb = self.gen_block(bb);

        // Insert the branch to it
        self.builder.build_unconditional_branch(bb).unwrap();
    }

    fn gen_call(&mut self, function: &FunctionIdx) -> IntValue<'ink> {
        // Ensure the function is compiled
        let function = self.functions.get(function).unwrap();

        self.builder
            .build_call(function.to_owned(), &[], "call some function")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value()
    }

    fn gen_return(&self, value: &Value) {
        let value = self.retrieve_value(value);

        self.builder
            .build_return(value.as_ref().map(|value| value as &dyn BasicValue))
            .unwrap();
    }

    fn gen_assign(&self, symbol: &Symbol, value: &Value) {
        let value = self
            .retrieve_value(&value)
            .expect("unit value cannot be assigned");
        let ptr = self.symbols.get(symbol).unwrap();

        self.builder.build_store(*ptr, value).unwrap();
    }

    fn gen_switch(
        &mut self,
        value: &Value,
        default: (BasicBlockIdx, Value),
        branches: Vec<(Value, BasicBlockIdx, Value)>,
    ) -> Option<IntValue<'ink>> {
        // Emit the switch instruction
        let cases = branches
            .iter()
            .map(|(case, bb, _)| {
                (
                    // Compile the case value
                    self.retrieve_value(case)
                        .expect("cannot use unit value as switch case"),
                    // Compile the destination basic block
                    self.gen_block(bb),
                )
            })
            .collect::<Vec<_>>();

        let else_block = self.gen_block(&default.0);

        self.builder
            .build_switch(
                // Build the value to switch on
                self.retrieve_value(&value)
                    .expect("cannot switch on unit value"),
                // Compile out the default branch
                else_block,
                &cases,
            )
            .unwrap();

        // Only create the phi node if the branches aren't unit value
        let make_phi = branches
            .first()
            // TODO: This check should probably be less-specific
            .map(|(_, _, value)| !matches!(value, Value::Unit))
            .unwrap_or(false);
        assert!(
            branches
                .iter()
                .all(|(_, _, value)| matches!(value, Value::Unit) != make_phi),
            "all or none of the branches must be unit, not a mix"
        );

        if make_phi {
            // Create the block to merge
            let merge = self
                .llvm_ctx
                .append_basic_block(self.llvm_function, "merge");

            // Position at merge block to create phi node
            self.builder.position_at_end(merge);
            let phi = self
                .builder
                .build_phi(self.llvm_ctx.i64_type(), "switch phi")
                .unwrap();

            // Build merge nodes
            for (bb, final_value) in branches
                .iter()
                .map(|(_, bb, final_value)| (*bb, *final_value))
                .chain([default])
            {
                // Pull the block out
                let basic_block = self.gen_block(&bb);

                // Ensure the block doesn't have a terminator
                if basic_block.get_terminator().is_some() {
                    panic!("basic block already has terminator, but one needs to be inserted for switch.");
                }

                // Add the terminator
                self.builder.position_at_end(basic_block);
                self.builder.build_unconditional_branch(merge).unwrap();

                // Update the phi node
                phi.add_incoming(&[(
                    &self
                        .retrieve_value(&final_value)
                        .expect("value to be not unit"),
                    basic_block,
                )]);
            }

            // Continue appending to end of merge block
            self.builder.position_at_end(merge);

            Some(phi.as_basic_value().into_int_value())
        } else {
            None
        }
    }

    /// Emit an allocation instruction in the entry basic block
    fn alloca(&self, ty: Ty, name: &str) -> PointerValue<'ink> {
        // Find the entry for this function
        let entry = self.llvm_function.get_first_basic_block().unwrap();

        // Create and position a builder to the end of the entry
        let builder = self.llvm_ctx.create_builder();
        builder.position_at_end(entry);

        builder
            .build_alloca(
                match ty {
                    Ty::Int => self.llvm_ctx.i64_type(),
                    Ty::Boolean => todo!(),
                    Ty::Unit => todo!(),
                },
                name,
            )
            .unwrap()
    }

    fn retrieve_value(&self, value: &Value) -> Option<IntValue<'ink>> {
        match value {
            Value::Name(symbol) => {
                let ptr = self.symbols.get(symbol).expect("symbol must be defined");
                Some(
                    self.builder
                        .build_load(self.llvm_ctx.i64_type(), *ptr, &format!("load {symbol:?}"))
                        .unwrap()
                        .into_int_value(),
                )
            }
            Value::Constant(value) => Some(match value {
                ConstantValue::Integer(value) => {
                    self.llvm_ctx.i64_type().const_int(*value as u64, false)
                }
                ConstantValue::Boolean(value) => {
                    let ty = self.llvm_ctx.bool_type();

                    if *value {
                        ty.const_all_ones()
                    } else {
                        ty.const_zero()
                    }
                }
            }),
            Value::Triple(triple) => Some(
                self.results
                    .get(triple.triple)
                    .expect("triple must exist")
                    .expect("triple must produce value"),
            ),
            Value::Unit => None,
        }
    }
}

pub trait FunctionGeneratorCtx: SymbolMap<Symbol = Symbol> {}
