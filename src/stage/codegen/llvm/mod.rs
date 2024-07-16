use std::collections::HashMap;

use index_vec::{IndexSlice, IndexVec};
use inkwell::{
    builder::Builder,
    context::Context as LLVMContext,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::{BasicValue, FunctionValue, IntValue, PointerValue},
    IntPredicate, OptimizationLevel,
};

use crate::{
    ctx::Symbol,
    repr::ir::{BinaryOp, Triple, UnaryOp, Value},
    stage::lower_ir::{FunctionIdx, IRCtx},
};

use crate::repr::ir::{BasicBlockIdx, ConstantValue, TripleIdx};

pub struct Pass<'ctx> {
    llvm_ctx: &'ctx LLVMContext,
    ir_ctx: IRCtx,
    module: Module<'ctx>,

    symbols: HashMap<Symbol, PointerValue<'ctx>>,
    basic_blocks: HashMap<(Symbol, BasicBlockIdx), inkwell::basic_block::BasicBlock<'ctx>>,
    pub function_values: HashMap<Symbol, FunctionValue<'ctx>>,
}

impl<'ctx> Pass<'ctx> {
    pub fn new(llvm_ctx: &'ctx LLVMContext, ir_ctx: IRCtx) -> Self {
        let module = llvm_ctx.create_module("module");

        Self {
            function_values: HashMap::from_iter(ir_ctx.functions.iter().map(|(idx, function)| {
                (*idx, {
                    // Forward-declare all the functions
                    // TODO: Pick appropriate return type depending on signature
                    let fn_type = llvm_ctx.i64_type().fn_type(&[], false);
                    let fn_value = module.add_function(
                        // TODO: Determine function name from identifier
                        // ir_ctx.symbol_map.resolve(function.symbol).unwrap(),
                        "my function",
                        fn_type,
                        None,
                    );

                    fn_value
                })
            })),
            llvm_ctx,
            ir_ctx,
            module,
            // TODO: Should be scoped to each function
            symbols: HashMap::new(),
            basic_blocks: HashMap::new(),
        }
    }

    /// Compile the provided function, returning the LLVM handle to it.
    pub fn compile(&mut self, function_id: Symbol) -> FunctionValue<'ctx> {
        let function = self
            .ir_ctx
            .functions
            .get(&function_id)
            .expect("function to exist");

        let builder = self.llvm_ctx.create_builder();

        let fn_value = self
            .function_values
            .get(&function_id)
            .expect("function to exist")
            .to_owned();

        // BUG: This won't work with multiple functions
        let entry_bb = (function_id, BasicBlockIdx::new(0));
        let entry = *self.basic_blocks.entry(entry_bb).or_insert_with(|| {
            self.llvm_ctx
                .append_basic_block(fn_value, &format!("bb_{:?}_{:?}", entry_bb.0, entry_bb.1))
        });
        builder.position_at_end(entry);

        // Create locals for all symbols
        // TODO: Symbols should be scoped to each function
        self.symbols.extend(function.scope.iter().map(|symbol| {
            (
                symbol.to_owned(),
                builder
                    .build_alloca(
                        self.llvm_ctx.i64_type(),
                        self.ir_ctx
                            .symbol_map
                            .resolve(*symbol)
                            .expect("symbol to exist in map"),
                    )
                    .unwrap(),
            )
        }));

        self.compile_basic_block(&fn_value, entry_bb);

        fn_value
    }

    fn compile_basic_block(
        &mut self,
        function: &FunctionValue<'ctx>,
        basic_block_id: (Symbol, BasicBlockIdx),
    ) {
        let bb = *self.basic_blocks.entry(basic_block_id).or_insert_with(|| {
            self.llvm_ctx
                .append_basic_block(*function, format!("bb_{basic_block_id:?}").as_str())
        });

        let builder = self.llvm_ctx.create_builder();
        builder.position_at_end(bb);

        let basic_block = self
            .ir_ctx
            .functions
            .get(&basic_block_id.0)
            .expect("function to exist")
            .basic_blocks
            .get(basic_block_id.1)
            .expect("requested basic block must exist")
            .clone();

        let mut results = IndexVec::with_capacity(basic_block.triples.len());

        for triple in &basic_block.triples {
            let result = match triple {
                Triple::BinaryOp { lhs, rhs, op } => {
                    let lhs = self
                        .retrive_value(&builder, &results, lhs)
                        .expect("lhs of binary op cannot be unit");
                    let rhs = self
                        .retrive_value(&builder, &results, rhs)
                        .expect("rhs of binary cannot be unit");

                    Some(match op {
                        BinaryOp::Add => builder.build_int_add(lhs, rhs, "add").unwrap(),
                        BinaryOp::Sub => builder.build_int_sub(lhs, rhs, "sub").unwrap(),
                        BinaryOp::Eq => builder
                            .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq")
                            .unwrap(),
                        BinaryOp::NotEq => builder
                            .build_int_compare(IntPredicate::NE, lhs, rhs, "not eq")
                            .unwrap(),
                    })
                }
                Triple::UnaryOp { rhs, op } => {
                    let rhs = self
                        .retrive_value(&builder, &results, rhs)
                        .expect("rhs of unary cannot be unit");

                    Some(match op {
                        UnaryOp::Minus => builder.build_int_neg(rhs, "neg").unwrap(),
                        UnaryOp::Not => builder.build_not(rhs, "not").unwrap(),
                    })
                }
                Triple::Copy(value) => Some(
                    self.retrive_value(&builder, &results, value)
                        .expect("cannot copy unit value"),
                ),
                Triple::Jump(bb) => {
                    // TODO: Better mapping between IR basic block and LLVM basic block
                    let bb = *self
                        .basic_blocks
                        .entry((basic_block_id.0, *bb))
                        .or_insert_with(|| {
                            self.llvm_ctx.append_basic_block(
                                *function,
                                format!("bb_{}", bb.index()).as_str(),
                            )
                        });
                    builder.build_unconditional_branch(bb).unwrap();

                    None
                }
                Triple::Call(call) => {
                    let function = self.function_values.get(call).unwrap();

                    Some(
                        builder
                            .build_call(function.to_owned(), &[], "call some function")
                            .unwrap()
                            .try_as_basic_value()
                            .unwrap_left()
                            .into_int_value(),
                    )
                }
                Triple::Return(value) => {
                    let value = self.retrive_value(&builder, &results, value);

                    builder
                        .build_return(value.as_ref().map(|value| value as &dyn BasicValue))
                        .unwrap();

                    None
                }
                Triple::Assign(symbol, value) => {
                    let value = self
                        .retrive_value(&builder, &results, value)
                        .expect("unit value cannot be assigned");
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
                        // TODO: Mapping bb
                        if !self.basic_blocks.contains_key(&(basic_block_id.0, bb)) {
                            self.compile_basic_block(function, (basic_block_id.0, bb));
                        }
                    }

                    // Emit the switch instruction
                    builder
                        .build_switch(
                            self.retrive_value(&builder, &results, value)
                                .expect("cannot switch on unit value"),
                            // TODO: Mapping bb
                            *self
                                .basic_blocks
                                .get(&(basic_block_id.0, default.0))
                                .unwrap(),
                            branches
                                .iter()
                                .map(|(case, bb, _)| {
                                    (
                                        self.retrive_value(&builder, &results, case)
                                            .expect("cannot use unit value as switch case"),
                                        // TODO: Mapping bb
                                        *self.basic_blocks.get(&(basic_block_id.0, *bb)).unwrap(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_slice(),
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
                        let merge = self.llvm_ctx.append_basic_block(*function, "switch merge");

                        // Position at merge block to create phi node
                        builder.position_at_end(merge);
                        let phi = builder
                            .build_phi(self.llvm_ctx.i64_type(), "switch phi")
                            .unwrap();

                        // Build merge nodes
                        for (bb, final_value) in branches
                            .iter()
                            .map(|(_, bb, final_value)| (*bb, *final_value))
                            .chain([*default])
                        {
                            // Pull the block out
                            // TODO: Mapping bb
                            let basic_block =
                                *self.basic_blocks.get(&(basic_block_id.0, bb)).unwrap();

                            // Ensure the block doesn't have a terminator
                            if basic_block.get_terminator().is_some() {
                                panic!("basic block already has terminator, but one needs to be inserted for switch.");
                            }

                            // Add the terminator
                            builder.position_at_end(basic_block);
                            builder.build_unconditional_branch(merge).unwrap();

                            // Update the phi node
                            phi.add_incoming(&[(
                                &self
                                    .retrive_value(&builder, &results, &final_value)
                                    .expect("value to be not unit"),
                                *self.basic_blocks.get(&(basic_block_id.0, bb)).unwrap(),
                            )]);
                        }

                        // Continue appending to end of merge block
                        builder.position_at_end(merge);

                        Some(phi.as_basic_value().into_int_value())
                    } else {
                        None
                    }
                }
            };

            results.push(result);
        }
    }

    fn retrive_value(
        &self,
        builder: &Builder<'ctx>,
        results: &IndexSlice<TripleIdx, [Option<IntValue<'ctx>>]>,
        value: &Value,
    ) -> Option<IntValue<'ctx>> {
        match value {
            Value::Name(symbol) => {
                let ptr = self.symbols.get(symbol).expect("symbol must be defined");
                Some(
                    builder
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
                results
                    .get(triple.triple)
                    .expect("triple must exist")
                    .expect("triple must produce value"),
            ),
            Value::Unit => None,
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
