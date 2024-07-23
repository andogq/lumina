mod ctx;

use std::collections::HashMap;

use index_vec::{IndexSlice, IndexVec};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context as LLVMContext,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    types::BasicType,
    values::{BasicValue, FunctionValue, IntValue, PointerValue},
    IntPredicate, OptimizationLevel,
};

pub use self::ctx::*;

use crate::{
    repr::{
        identifier::FunctionIdx,
        ir::{BasicBlockIdx, BinaryOp, ConstantValue, Triple, TripleIdx, UnaryOp, Value},
        ty::Ty,
    },
    stage::lower_ir::IRCtx,
    util::symbol_map::interner_symbol_map::Symbol,
};

pub struct LLVMCodegenPass<'ctx, Ctx> {
    ctx: Ctx,
    llvm_ctx: LLVMContext,

    function_values: HashMap<FunctionIdx, FunctionValue<'ctx>>,
    module: Module<'ctx>,
}

impl<'ctx, Ctx> LLVMCodegenPass<'ctx, Ctx>
where
    Ctx: LLVMCodegenCtx<'ctx>,
{
    pub fn compile_function(&mut self, function_idx: FunctionIdx) -> FunctionValue<'ctx> {
        // Early terminate if this function has already been compiled
        if let Some(function_value) = self.lookup_function_value(function_idx) {
            return function_value;
        }

        // Fetch the IR for this function
        let function = self.ctx.get_function(function_idx);

        // Create the underlying LLVM value for this function.
        let function_value = self.create_function_value(function_idx);

        // Create the entry point for this function
        let entry = ctx.append_basic_block(function_idx);

        // Position the builder at the entry point
        let builder = ctx.new_builder();
        builder.position_at_end(entry);

        // Prepare all locals within this function
        // TODO: This should have a representation in the IR so it can be done in a basic block
        let symbol_locations = function
            .scope
            .iter()
            .map(|symbol| {
                (
                    *symbol,
                    builder
                        .build_alloca(
                            // TODO: Actually determine the type of this symbol
                            ctx.get_type(Ty::Int),
                            &ctx.get(*symbol),
                        )
                        .unwrap(),
                )
            })
            .collect::<HashMap<_, _>>();

        // Create the builder context
        let mut builder_ctx = ctx.create_function_builder(symbol_locations);

        // Compile the user's entry, and capture it
        let user_entry = compile_basic_block(
            ctx,
            &mut builder_ctx,
            // WARN: Convoluted way of getting the idx of the first basic block
            function.basic_blocks.indices().next().unwrap(),
        );

        // Add a jump in from the function's entry to get to the user's
        builder.build_unconditional_branch(user_entry).unwrap();

        // Produce the resulting function value
        function_value
    }

    fn compile_basic_block<'ctx>(
        ctx: &mut impl LLVMCodegenCtx<'ctx>,
        builder_ctx: &mut impl LLVMFunctionBuilder<'ctx>,
        basic_block_idx: BasicBlockIdx,
    ) -> BasicBlock<'ctx> {
        // If this basic block has already been compiled, bail early
        if let Some(basic_block) = builder_ctx.lookup_basic_block(basic_block_idx) {
            return basic_block;
        }

        // Create the basic block
        let basic_block = builder_ctx.create_basic_block(basic_block_idx);

        // Create and position a new builder in the basic block
        let builder = ctx.new_builder();
        builder.position_at_end(basic_block);

        // Track the results of each triple, so they can be referenced at a later point
        let mut results = IndexVec::<TripleIdx, Option<IntValue>>::new();

        for triple in builder_ctx.get_triples(basic_block_idx) {
            let result = match triple {
                Triple::BinaryOp { lhs, rhs, op } => {
                    let lhs = retrieve_value(ctx, builder_ctx, &builder, &results, &lhs)
                        .expect("lhs of binary op cannot be unit");
                    let rhs = retrieve_value(ctx, builder_ctx, &builder, &results, &rhs)
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
                    let rhs = retrieve_value(ctx, builder_ctx, &builder, &results, &rhs)
                        .expect("rhs of unary cannot be unit");

                    Some(match op {
                        UnaryOp::Minus => builder.build_int_neg(rhs, "neg").unwrap(),
                        UnaryOp::Not => builder.build_not(rhs, "not").unwrap(),
                    })
                }
                Triple::Copy(value) => Some(
                    retrieve_value(ctx, builder_ctx, &builder, &results, &value)
                        .expect("cannot copy unit value"),
                ),
                Triple::Jump(bb) => {
                    // Ensure the basic block is compiled
                    let bb = compile_basic_block(ctx, builder_ctx, bb);

                    // Insert the branch to it
                    builder.build_unconditional_branch(bb).unwrap();

                    None
                }
                Triple::Call(call) => {
                    // Ensure the function is compiled
                    let function = compile_function(ctx, call);

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
                    let value = retrieve_value(ctx, builder_ctx, &builder, &results, &value);

                    builder
                        .build_return(value.as_ref().map(|value| value as &dyn BasicValue))
                        .unwrap();

                    None
                }
                Triple::Assign(symbol, value) => {
                    let value = retrieve_value(ctx, builder_ctx, &builder, &results, &value)
                        .expect("unit value cannot be assigned");
                    let ptr = builder_ctx.lookup_symbol(symbol);

                    builder.build_store(ptr, value).unwrap();

                    None
                }
                Triple::Switch {
                    value,
                    default,
                    branches,
                } => {
                    // Emit the switch instruction
                    builder
                        .build_switch(
                            // Build the value to switch on
                            retrieve_value(ctx, builder_ctx, &builder, &results, &value)
                                .expect("cannot switch on unit value"),
                            // Compile out the default branch
                            compile_basic_block(ctx, builder_ctx, default.0),
                            branches
                                .iter()
                                .map(|(case, bb, _)| {
                                    (
                                        // Compile the case value
                                        retrieve_value(ctx, builder_ctx, &builder, &results, case)
                                            .expect("cannot use unit value as switch case"),
                                        // Compile the destination basic block
                                        compile_basic_block(ctx, builder_ctx, *bb),
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
                        let merge = builder_ctx.anonymous_basic_block();

                        // Position at merge block to create phi node
                        builder.position_at_end(merge);
                        let phi = builder
                            .build_phi(ctx.get_type(Ty::Int), "switch phi")
                            .unwrap();

                        // Build merge nodes
                        for (bb, final_value) in branches
                            .iter()
                            .map(|(_, bb, final_value)| (*bb, *final_value))
                            .chain([default])
                        {
                            // Pull the block out
                            let basic_block = compile_basic_block(ctx, builder_ctx, bb);

                            // Ensure the block doesn't have a terminator
                            if basic_block.get_terminator().is_some() {
                                panic!("basic block already has terminator, but one needs to be inserted for switch.");
                            }

                            // Add the terminator
                            builder.position_at_end(basic_block);
                            builder.build_unconditional_branch(merge).unwrap();

                            // Update the phi node
                            phi.add_incoming(&[(
                                &retrieve_value(ctx, builder_ctx, &builder, &results, &final_value)
                                    .expect("value to be not unit"),
                                basic_block,
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

        basic_block
    }

    fn retrieve_value<'ctx>(
        ctx: &impl LLVMCodegenCtx<'ctx>,
        builder_ctx: &impl LLVMFunctionBuilder<'ctx>,
        builder: &Builder<'ctx>,
        results: &IndexSlice<TripleIdx, [Option<IntValue<'ctx>>]>,
        value: &Value,
    ) -> Option<IntValue<'ctx>> {
        match value {
            Value::Name(symbol) => {
                let ptr = builder_ctx.lookup_symbol(*symbol);
                Some(
                    builder
                        .build_load(ctx.get_type(Ty::Int), ptr, &format!("load {symbol:?}"))
                        .unwrap()
                        .into_int_value(),
                )
            }
            Value::Constant(value) => Some(match value {
                ConstantValue::Integer(value) => ctx.const_int(*value),
                ConstantValue::Boolean(value) => ctx.const_bool(*value),
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

    /// Get the LLVM function value for the provided function.
    fn lookup_function_value(&self, function_idx: FunctionIdx) -> Option<FunctionValue<'ctx>> {
        self.function_values.get(&function_idx).cloned()
    }

    /// Create a new LLVM function value for the provided function.
    fn create_function_value(&mut self, function_idx: FunctionIdx) -> FunctionValue<'ctx> {
        if let Some(value) = self.lookup_function_value(function_idx) {
            return value;
        }

        // Fetch the signature of the function
        let function = self.ctx.get_function(function_idx);

        let value = {
            let ty = self.get_type(function.signature.return_ty).fn_type(
                function
                    .signature
                    .arguments
                    .iter()
                    .map(|arg| self.get_type(*arg).as_basic_type_enum().into())
                    .collect::<Vec<_>>()
                    .as_slice(),
                false,
            );

            self.module.add_function("function_name", ty, None)
        };

        self.function_values.insert(function_idx, value);

        value
    }

    /// Fetch the LLVM type for the corresponding IR type
    fn get_type(&self, ty: Ty) -> impl BasicType<'ctx> {
        match ty {
            Ty::Int => todo!(),
            Ty::Boolean => todo!(),
            Ty::Unit => todo!(),
        }
    }
}

pub struct Pass<'ctx, I> {
    llvm_ctx: &'ctx LLVMContext,
    ir_ctx: I,
    module: Module<'ctx>,

    symbols: HashMap<Symbol, PointerValue<'ctx>>,
    basic_blocks: HashMap<(FunctionIdx, BasicBlockIdx), inkwell::basic_block::BasicBlock<'ctx>>,
    pub function_values: HashMap<FunctionIdx, FunctionValue<'ctx>>,
}

impl<'ctx, I> Pass<'ctx, I>
where
    I: IRCtx,
{
    pub fn new(llvm_ctx: &'ctx LLVMContext, ir_ctx: I) -> Self {
        let module = llvm_ctx.create_module("module");

        Self {
            function_values: HashMap::from_iter(ir_ctx.all_functions().iter().map(
                |(idx, _function)| {
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
                },
            )),
            llvm_ctx,
            ir_ctx,
            module,
            // TODO: Should be scoped to each function
            symbols: HashMap::new(),
            basic_blocks: HashMap::new(),
        }
    }

    /// Compile the provided function, returning the LLVM handle to it.
    pub fn compile(&mut self, function_idx: FunctionIdx) -> FunctionValue<'ctx> {
        let function = self
            .ir_ctx
            .all_functions()
            .iter()
            .find(|(idx, _)| idx == &function_idx)
            .expect("function to exist")
            .1
            .clone();

        let builder = self.llvm_ctx.create_builder();

        let fn_value = self
            .function_values
            .get(&function_idx)
            .expect("function to exist")
            .to_owned();

        // BUG: This won't work with multiple functions
        let entry_bb = (function_idx, BasicBlockIdx::new(0));
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
                    .build_alloca(self.llvm_ctx.i64_type(), "todo: work out symbol name")
                    .unwrap(),
            )
        }));

        self.compile_basic_block(&fn_value, entry_bb);

        fn_value
    }

    fn compile_basic_block(
        &mut self,
        function: &FunctionValue<'ctx>,
        basic_block_id: (FunctionIdx, BasicBlockIdx),
    ) {
        let bb = *self.basic_blocks.entry(basic_block_id).or_insert_with(|| {
            self.llvm_ctx
                .append_basic_block(*function, format!("bb_{basic_block_id:?}").as_str())
        });

        let builder = self.llvm_ctx.create_builder();
        builder.position_at_end(bb);

        let basic_block = self
            .ir_ctx
            .all_functions()
            .iter()
            .find(|(idx, _)| idx == &basic_block_id.0)
            .expect("function to exist")
            .1
            .clone()
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
