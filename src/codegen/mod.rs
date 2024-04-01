use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::{FunctionValue, IntValue},
    OptimizationLevel,
};

use crate::core::ast::{Expression, Function, InfixOperation, Program, Statement};

pub struct Compiler {
    context: Context,
}

impl<'ctx> Compiler {
    pub fn new() -> Self {
        Self {
            context: Context::create(),
        }
    }

    pub fn compile(&'ctx self, program: Program) -> CompiledModule<'ctx> {
        let pass = CompilePass {
            context: &self.context,
            module: self.context.create_module("module"),
            builder: self.context.create_builder(),
        };

        // Compile each of the individual functions
        for function in program.functions {
            pass.compile_function(function);
        }

        // Compile the main function
        let main = pass.compile_function(program.main);

        let module = CompiledModule {
            module: pass.module,
            main,
        };

        module.run_passes();

        module
    }
}

struct CompilePass<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CompilePass<'ctx> {
    fn compile_expression(&self, expression: Expression) -> IntValue {
        match expression {
            Expression::Infix(infix) => {
                let left = self.compile_expression(*infix.left);
                let right = self.compile_expression(*infix.right);

                match infix.operation {
                    InfixOperation::Plus(_) => {
                        self.builder.build_int_add(left, right, "temp_add").unwrap()
                    }
                }
            }
            Expression::Integer(integer) => self
                .context
                .i64_type()
                .const_int(integer.literal as u64, true),
        }
    }

    fn compile_statement(&self, statement: Statement) {
        match statement {
            Statement::Return(s) => {
                let value = self.compile_expression(s.value);
                self.builder.build_return(Some(&value)).unwrap();
            }
            Statement::Expression(s) => {
                self.compile_expression(s.expression);
            }
        };
    }

    fn compile_function(&self, function: Function) -> FunctionValue<'ctx> {
        // Create a prototype
        let fn_type = self.context.i64_type().fn_type(&[], false);
        let fn_value = self.module.add_function(&function.name, fn_type, None);

        // Create the entry point and position the builder
        let entry = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry);

        // Compile the body
        for statement in function.body {
            self.compile_statement(statement);
        }

        // Verify and optimise the function
        fn_value.verify(true);

        fn_value
    }
}

pub struct CompiledModule<'ctx> {
    module: Module<'ctx>,
    main: FunctionValue<'ctx>,
}

impl<'ctx> CompiledModule<'ctx> {
    fn run_passes(&self) {
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

        let passes = &[
            "instcombine",
            "reassociate",
            "gvn",
            "simplifycfg",
            "mem2reg",
        ];

        self.module
            .run_passes(
                passes.join(",").as_str(),
                &target_machine,
                PassBuilderOptions::create(),
            )
            .unwrap();
    }

    pub fn jit(&self) {
        let engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let main = self.main.get_name().to_str().unwrap();

        match unsafe { engine.get_function::<unsafe extern "C" fn() -> i64>(main) } {
            Ok(f) => {
                // run function
                let result = unsafe { f.call() };
                println!("{result}");
            }
            Err(e) => {
                eprintln!("unable to execute function: {e:?}");
            }
        }
    }
}
