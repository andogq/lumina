use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context as LLVMContext,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    types::PointerType,
    values::{BasicValue, FunctionValue, IntValue, PointerValue},
    AddressSpace, OptimizationLevel,
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

        // Create the entry basic block
        // let entry = self.llvm_ctx.append_basic_block(fn_value, "entry");
        // builder.position_at_end(entry);

        // self.compile_basic_block(&fn_value, function_id);

        let entry = *self
            .basic_blocks
            .entry(0)
            .or_insert_with(|| self.llvm_ctx.append_basic_block(fn_value, "bb_0"));
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

        for bb in 0..self.ir_ctx.basic_blocks.len() {
            self.compile_basic_block(&fn_value, bb);
        }

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
            .expect("requested basic block must exist");

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
                Triple::CondJump(condition, then_block, else_block) => {
                    let condition = self.retrive_value(&builder, &results, condition);
                    let then_block = *self.basic_blocks.entry(*then_block).or_insert_with(|| {
                        self.llvm_ctx
                            .append_basic_block(*function, format!("bb_{then_block}").as_str())
                    });
                    let else_block = *self.basic_blocks.entry(*else_block).or_insert_with(|| {
                        self.llvm_ctx
                            .append_basic_block(*function, format!("bb_{else_block}").as_str())
                    });

                    builder
                        .build_conditional_branch(condition, then_block, else_block)
                        .unwrap();

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
                Triple::CreatePhi(incoming) => {
                    let phi = builder.build_phi(self.llvm_ctx.i64_type(), "phi").unwrap();

                    // WARN: This is horrific
                    let incoming_owned = incoming
                        .iter()
                        .map(|(value, bb_id)| {
                            let value = self.retrive_value(&builder, &results, value);
                            let bb = self.basic_blocks.get(bb_id).expect("basic block to exist");

                            (value, *bb)
                        })
                        .collect::<Vec<_>>();
                    let incoming = incoming_owned
                        .iter()
                        .map(|(value, bb)| (value as &dyn BasicValue, *bb))
                        .collect::<Vec<_>>();

                    phi.add_incoming(incoming.as_slice());

                    let value = phi.as_basic_value().into_int_value();
                    // let value = builder
                    //     .build_load(self.llvm_ctx.i64_type(), ptr, "load phi")
                    //     .unwrap()
                    //     .into_int_value();

                    // TODO: This is pushing the pointer value, not the value at the
                    // pointer (requires a load);
                    Some(value)
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
