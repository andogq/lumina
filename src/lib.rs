use codegen::llvm::Module;
use compiler::Compiler;
use inkwell::{
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::FunctionValue,
    OptimizationLevel,
};

pub mod codegen;
pub mod compiler;
pub mod repr;
pub mod stage;
pub mod util;

pub fn compile_and_run(source: &'static str, debug: bool) -> i64 {
    // Create compiler state
    let mut compiler = Compiler::default();

    // Create LLVM state
    let llvm_ctx = inkwell::context::Context::create();

    // Compile the source to produce all the functions
    let functions = compiler.compile(source).unwrap();

    // Create an LLVM module from the compiler and an LLVM instance
    let module = Module::new(&compiler, &llvm_ctx);

    // Compile each of the functions into the LLVM module, and capture the corresponding value
    let functions = functions
        .iter()
        .map(|function| (function.identifier, module.compile(function)))
        .collect::<Vec<_>>();

    // Find the main function
    let main = {
        let main_symbol = compiler.symbols.get("main").unwrap();

        functions
            .iter()
            .find(|(identifier, _)| {
                compiler.get_function_symbol(*identifier).unwrap() == main_symbol
            })
            .map(|(_, function)| function)
            .unwrap()
    };

    // Pull out the inner LLVM module
    let module = module.into_inner();

    if debug {
        module.print_to_stderr();
    } else {
        run_passes(
            &module,
            &[
                "instcombine",
                "reassociate",
                "gvn",
                "simplifycfg",
                "mem2reg",
            ],
        );
    }

    jit(&module, *main)
}

#[allow(unused)]
fn run_passes(module: &inkwell::module::Module, passes: &[&str]) {
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

    module
        .run_passes(
            passes.join(",").as_str(),
            &target_machine,
            PassBuilderOptions::create(),
        )
        .unwrap();
}

fn jit(module: &inkwell::module::Module, entry: FunctionValue) -> i64 {
    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        engine
            .get_function::<unsafe extern "C" fn() -> i64>(entry.get_name().to_str().unwrap())
            .unwrap()
            .call()
    }
}
