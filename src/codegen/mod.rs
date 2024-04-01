use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::{FunctionValue, IntValue, PointerValue},
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
    fn compile_expression(
        &self,
        expression: Expression,
        symbol_table: &HashMap<String, PointerValue<'ctx>>,
    ) -> IntValue {
        match expression {
            Expression::Infix(infix) => {
                let left = self.compile_expression(*infix.left, symbol_table);
                let right = self.compile_expression(*infix.right, symbol_table);

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
            Expression::Ident(ident) => self
                .builder
                .build_load(
                    self.context.i64_type(),
                    symbol_table.get(&ident.name).unwrap().clone(),
                    &ident.name,
                )
                .unwrap()
                .into_int_value(),
            Expression::Boolean(boolean) => {
                if boolean.literal {
                    self.context.bool_type().const_all_ones()
                } else {
                    self.context.bool_type().const_zero()
                }
            }
        }
    }

    fn compile_statement(
        &self,
        statement: Statement,
        symbol_table: &mut HashMap<String, PointerValue<'ctx>>,
    ) {
        match statement {
            Statement::Return(s) => {
                let value = self.compile_expression(s.value, symbol_table);
                self.builder.build_return(Some(&value)).unwrap();
            }
            Statement::Expression(s) => {
                self.compile_expression(s.expression, symbol_table);
            }
            Statement::Let(s) => {
                // Compile value of let statement
                let value = self.compile_expression(s.value, symbol_table);

                // Create a place for the variable to be stored
                let entry = self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap()
                    .get_first_basic_block()
                    .unwrap();

                // Create a new builder to not change the position of the current builder
                let stack_builder = self.context.create_builder();

                // Position builder to be at the start of the entry block
                match entry.get_first_instruction() {
                    Some(instr) => stack_builder.position_before(&instr),
                    None => stack_builder.position_at_end(entry),
                };

                // Stack address that this variable will be stored at
                let addr = self
                    .builder
                    .build_alloca(self.context.i64_type(), &s.name)
                    .unwrap();

                // Move statement value onto stack
                self.builder.build_store(addr, value).unwrap();

                // Add address to the symbol table
                symbol_table.insert(s.name, addr);
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

        let mut symbol_table = HashMap::new();

        // Compile the body
        for statement in function.body {
            self.compile_statement(statement, &mut symbol_table);
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
