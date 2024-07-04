use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context as LLVMContext,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::{FunctionValue, IntValue, PointerValue},
    OptimizationLevel,
};

use crate::{
    codegen::ir::{BinaryOp, IRContext, Triple, UnaryOp, Value},
    core::symbol::Symbol,
};

pub struct Pass<'ctx> {
    llvm_ctx: &'ctx LLVMContext,
    ir_ctx: IRContext,
    module: Module<'ctx>,

    symbols: HashMap<Symbol, PointerValue<'ctx>>,
    basic_blocks: HashMap<usize, inkwell::basic_block::BasicBlock<'ctx>>,
}

impl<'ctx> Pass<'ctx> {
    pub fn new(llvm_ctx: &'ctx LLVMContext, ir_ctx: IRContext) -> Self {
        Self {
            llvm_ctx,
            ir_ctx,
            module: llvm_ctx.create_module("module"),
            symbols: HashMap::new(),
            basic_blocks: HashMap::new(),
        }
    }

    /// Compile the provided function, returning the LLVM handle to it.
    pub fn compile(&mut self, function_id: usize) -> FunctionValue<'ctx> {
        let builder = self.llvm_ctx.create_builder();

        // TODO: Currently only accepts functions that return an integer
        let fn_type = self.llvm_ctx.i64_type().fn_type(&[], false);
        let fn_value = self.module.add_function("some function", fn_type, None);

        let entry = *self.basic_blocks.entry(function_id).or_insert_with(|| {
            self.llvm_ctx
                .append_basic_block(fn_value, &format!("bb_{function_id}"))
        });
        builder.position_at_end(entry);

        // Create locals for all symbols
        self.symbols
            .extend(self.ir_ctx.symbols.iter().map(|symbol| {
                (
                    symbol.to_owned(),
                    builder
                        .build_alloca(self.llvm_ctx.i64_type(), &symbol.to_string())
                        .unwrap(),
                )
            }));

        self.compile_basic_block(&fn_value, function_id);

        fn_value
    }

    fn compile_basic_block(&mut self, function: &FunctionValue<'ctx>, basic_block_id: usize) {
        let bb = *self.basic_blocks.entry(basic_block_id).or_insert_with(|| {
            self.llvm_ctx
                .append_basic_block(*function, format!("bb_{basic_block_id}").as_str())
        });

        let builder = self.llvm_ctx.create_builder();
        builder.position_at_end(bb);

        let basic_block = self
            .ir_ctx
            .basic_blocks
            .get(basic_block_id)
            .expect("requested basic block must exist")
            .clone();

        let mut results = Vec::with_capacity(basic_block.triples.len());

        for triple in &basic_block.triples {
            let result = match triple {
                Triple::BinaryOp { lhs, rhs, op } => {
                    let lhs = self.retrive_value(&builder, &results, lhs);
                    let rhs = self.retrive_value(&builder, &results, rhs);

                    Some(match op {
                        BinaryOp::Add => builder.build_int_add(lhs, rhs, "add").unwrap(),
                        BinaryOp::Sub => builder.build_int_sub(lhs, rhs, "sub").unwrap(),
                    })
                }
                Triple::UnaryOp { rhs, op } => {
                    let rhs = self.retrive_value(&builder, &results, rhs);

                    Some(match op {
                        UnaryOp::Minus => builder.build_int_neg(rhs, "neg").unwrap(),
                        UnaryOp::Not => builder.build_not(rhs, "not").unwrap(),
                    })
                }
                Triple::Copy(value) => Some(self.retrive_value(&builder, &results, value)),
                Triple::Jump(bb) => {
                    let bb = *self.basic_blocks.entry(*bb).or_insert_with(|| {
                        self.llvm_ctx
                            .append_basic_block(*function, format!("bb_{bb}").as_str())
                    });
                    builder.build_unconditional_branch(bb).unwrap();

                    None
                }
                Triple::Call(_) => todo!(),
                Triple::Return(value) => {
                    let value = self.retrive_value(&builder, &results, value);

                    builder.build_return(Some(&value)).unwrap();

                    None
                }
                Triple::Assign(symbol, value) => {
                    let value = self.retrive_value(&builder, &results, value);
                    let ptr = self.symbols.get(symbol).expect("symbol must be defined");

                    builder.build_store(*ptr, value).unwrap();

                    None
                }
                Triple::Switch {
                    value,
                    default,
                    branches,
                } => {
                    // Ensure each of the required basic blocks are compiled
                    for bb in branches.iter().map(|(_, bb, _)| *bb).chain([default.0]) {
                        if !self.basic_blocks.contains_key(&bb) {
                            self.compile_basic_block(function, bb);
                        }
                    }

                    // Emit the switch instruction
                    builder
                        .build_switch(
                            self.retrive_value(&builder, &results, value),
                            *self.basic_blocks.get(&default.0).unwrap(),
                            branches
                                .iter()
                                .map(|(case, bb, _)| {
                                    (
                                        self.retrive_value(&builder, &results, case),
                                        *self.basic_blocks.get(bb).unwrap(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )
                        .unwrap();

                    // Create the block to merge
                    let merge = self.llvm_ctx.append_basic_block(*function, "switch merge");

                    // Write phi node into merge block
                    builder.position_at_end(merge);

                    // Build a phi node to receive the result of the switch
                    let phi = builder
                        .build_phi(self.llvm_ctx.i64_type(), "switch phi")
                        .unwrap();

                    for (bb, final_value) in branches
                        .iter()
                        .map(|(_, bb, final_value)| (*bb, *final_value))
                        .chain([*default])
                    {
                        // Pull the block out
                        let basic_block = *self.basic_blocks.get(&bb).unwrap();

                        // Ensure the block doesn't have a terminator
                        if basic_block.get_terminator().is_some() {
                            panic!("basic block already has terminator, but one needs to be inserted for switch.");
                        }

                        // Add the terminator
                        builder.position_at_end(basic_block);
                        builder.build_unconditional_branch(merge).unwrap();

                        phi.add_incoming(&[(
                            &self.retrive_value(&builder, &results, &final_value),
                            basic_block,
                        )]);
                    }

                    // Continue writing to end of merge block
                    builder.position_at_end(merge);

                    Some(phi.as_basic_value().into_int_value())
                }
            };

            results.push(result);
        }
    }

    fn retrive_value(
        &self,
        builder: &Builder<'ctx>,
        results: &[Option<IntValue<'ctx>>],
        value: &Value,
    ) -> IntValue<'ctx> {
        match value {
            Value::Name(symbol) => {
                let ptr = self.symbols.get(symbol).expect("symbol must be defined");
                builder
                    .build_load(self.llvm_ctx.i64_type(), *ptr, &format!("load {symbol}"))
                    .unwrap()
                    .into_int_value()
            }
            Value::Constant(value) => self.llvm_ctx.i64_type().const_int(*value as u64, false),
            Value::Triple(triple) => results
                .get(triple.triple)
                .expect("triple must exist")
                .expect("triple must produce value"),
        }
    }

    pub fn run_passes(&self, passes: &[&str]) {
        Target::initialize_all(&Default::default());

        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                OptimizationLevel::None,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .unwrap();

        self.module
            .run_passes(
                passes.join(",").as_str(),
                &target_machine,
                PassBuilderOptions::create(),
            )
            .unwrap();
    }

    pub fn jit(&self, entry: FunctionValue) -> i64 {
        let engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            engine
                .get_function::<unsafe extern "C" fn() -> i64>(entry.get_name().to_str().unwrap())
                .unwrap()
                .call()
        }
    }

    pub fn debug_print(&self) {
        self.module.print_to_stderr();
    }
}
