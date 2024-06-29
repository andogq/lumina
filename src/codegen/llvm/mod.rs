use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context as LLVMContext,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::{BasicValue, FunctionValue, IntValue, PointerValue},
    OptimizationLevel,
};

use crate::codegen::ir::{Statement, Terminator};

use super::ir::{
    value::{Local, RValue},
    BasicBlockData, BinaryOperation, Context, ContextInner, Function, RETURN_LOCAL,
};

pub mod llvm2;

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
        locals: Locals<'ctx>,
        basic_block: BasicBlockData,
    ) {
        let builder = self.llvm_ctx.create_builder();
        builder.position_at_end(target);

        let mut statement_results = vec![None; basic_block.statements.len()];

        for (i, statement) in basic_block.statements.into_iter().enumerate() {
            let mut result = None;

            match statement {
                Statement::Assign(local, value) => {
                    // Determine the pointer of the local that it will be saved in
                    let ptr = locals.get(&local).unwrap().to_owned();

                    // Determine the actual value
                    let value = self.rvalue_to_value(value, &builder, &locals, &statement_results);

                    // Emit the instruction
                    builder.build_store(ptr, value).unwrap();
                }
                Statement::Infix { lhs, rhs, op } => {
                    let lhs = self.rvalue_to_value(lhs, &builder, &locals, &statement_results);
                    let rhs = self.rvalue_to_value(rhs, &builder, &locals, &statement_results);

                    result = Some(match op {
                        BinaryOperation::Plus => builder.build_int_add(lhs, rhs, "add").unwrap(),
                    });
                }
            }

            statement_results[i] = result;
        }

        match basic_block.terminator {
            Terminator::Return(value) => {
                let value = self
                    .rvalue_to_value(value, &builder, &locals, &statement_results)
                    .as_basic_value_enum();

                builder.build_return(Some(&value)).unwrap();
            }
        }
    }

    fn rvalue_to_value(
        &self,
        value: RValue,
        builder: &Builder<'ctx>,
        locals: &Locals<'ctx>,
        statement_results: &[Option<IntValue<'ctx>>],
    ) -> IntValue<'ctx> {
        match value {
            RValue::Scalar(s) => {
                // TODO: Properly handle arbitrary precision
                self.llvm_ctx.i64_type().const_int(s.data, false)
            }
            RValue::Local(local) => builder
                .build_load(
                    self.llvm_ctx.i64_type(),
                    *locals.get(&local).unwrap(),
                    "load local",
                )
                .unwrap()
                .into_int_value(),
            RValue::Statement(statement) => statement_results
                .get(statement)
                .expect("statement result request must be for previous value")
                .expect("statement to have produced result"),
        }
    }
}
