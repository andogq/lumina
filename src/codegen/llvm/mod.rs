use std::collections::HashMap;

use inkwell::{
    context::Context as LLVMContext,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::{FunctionValue, PointerValue},
    OptimizationLevel,
};

use crate::codegen::ir::{Statement, Terminator};

use super::ir::{
    value::{Local, RValue},
    BasicBlockData, BinaryOperation, Context, ContextInner, Function, RETURN_LOCAL,
};

type Locals<'ctx> = HashMap<Local, PointerValue<'ctx>>;

pub struct Pass<'ctx> {
    llvm_ctx: &'ctx LLVMContext,
    ir_ctx: ContextInner,
    module: Module<'ctx>,
}

impl<'ctx> Pass<'ctx> {
    /// Create a LLVM pass to compile the IR into LLVM IR.
    pub fn new(llvm_ctx: &'ctx LLVMContext, ir_ctx: Context) -> Self {
        Self {
            llvm_ctx,
            ir_ctx: ir_ctx.into_inner(),
            module: llvm_ctx.create_module("module"),
        }
    }

    pub fn compile(&self, function: Function) -> FunctionValue<'ctx> {
        let builder = self.llvm_ctx.create_builder();

        // Create the prototype of the function
        // TODO: Currently only accepts functions that return an integer
        let fn_type = self.llvm_ctx.i64_type().fn_type(&[], false);
        let fn_value = self
            .module
            .add_function(&function.name.to_string(), fn_type, None);

        // Create the entry basic block
        let entry = self.llvm_ctx.append_basic_block(fn_value, "entry");
        builder.position_at_end(entry);

        // Prepare locals for the function body
        let mut locals = HashMap::new();
        locals.insert(RETURN_LOCAL, {
            builder
                .build_alloca(self.llvm_ctx.i64_type(), "return value")
                .unwrap()
        });

        locals.extend(function.scope.locals.iter().map(|local| {
            (
                local,
                builder
                    .build_alloca(self.llvm_ctx.i64_type(), "var")
                    .unwrap(),
            )
        }));

        self.compile_basic_block(
            entry,
            locals,
            self.ir_ctx
                .basic_blocks
                .get(function.entry)
                .unwrap()
                .to_owned(),
        );

        fn_value
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

    fn compile_basic_block(
        &self,
        target: inkwell::basic_block::BasicBlock,
        locals: Locals,
        basic_block: BasicBlockData,
    ) {
        let builder = self.llvm_ctx.create_builder();
        builder.position_at_end(target);

        for statement in basic_block.statements {
            match statement {
                Statement::Assign(local, value) => {
                    let ptr = locals.get(&local).unwrap().to_owned();

                    let value = match value {
                        RValue::Scalar(s) => {
                            // TODO: Properly handle arbitrary precision
                            self.llvm_ctx.i64_type().const_int(s.data, false)
                        }
                    };

                    builder.build_store(ptr, value).unwrap();
                }
                Statement::Load { result, target } => {
                    let val = builder
                        .build_load(
                            self.llvm_ctx.i64_type(),
                            locals.get(&target).unwrap().to_owned(),
                            "load var",
                        )
                        .unwrap();
                    builder
                        .build_store(locals.get(&result).unwrap().to_owned(), val)
                        .unwrap();
                }
                Statement::Infix {
                    lhs,
                    rhs,
                    op,
                    target,
                } => {
                    // WARN: These loads are incorrect, but do work
                    let lhs = builder
                        .build_load(
                            self.llvm_ctx.i64_type(),
                            *locals.get(&lhs).unwrap(),
                            "load lhs",
                        )
                        .unwrap()
                        .into_int_value();
                    let rhs = builder
                        .build_load(
                            self.llvm_ctx.i64_type(),
                            *locals.get(&rhs).unwrap(),
                            "load rhs",
                        )
                        .unwrap()
                        .into_int_value();

                    let result = match op {
                        BinaryOperation::Plus => builder.build_int_add(lhs, rhs, "add").unwrap(),
                    };

                    builder
                        .build_store(locals.get(&target).unwrap().to_owned(), result)
                        .unwrap();
                }
            }
        }

        match basic_block.terminator {
            Terminator::Return => {
                builder
                    .build_return(Some({
                        // TODO: Assumes that there is a return value
                        let ptr = locals.get(&RETURN_LOCAL).unwrap();

                        &builder
                            .build_load(self.llvm_ctx.i64_type(), *ptr, "load return")
                            .unwrap()
                    }))
                    .unwrap();
            }
        }
    }
}
