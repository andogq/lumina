mod expression;
mod function;
mod ir;
mod program;
mod statement;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::FunctionValue,
    OptimizationLevel,
};

use crate::core::{ast::Program, symbol::SymbolMap};

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
        let mut pass = CompilePass {
            context: &self.context,
            module: self.context.create_module("module"),
            builder: self.context.create_builder(),
            symbol_map: program.symbol_map,
        };

        // Compile each of the individual functions
        for function in program.functions {
            function.compile(&mut pass);
        }

        // Compile the main function
        let main = program.main.compile(&mut pass);

        let module = CompiledModule {
            module: pass.module,
            main,
        };

        module.run_passes();

        module
    }
}

pub struct CompilePass<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    symbol_map: SymbolMap,
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
