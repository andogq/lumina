use std::collections::HashMap;

use inkwell::{
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::FunctionValue,
    OptimizationLevel,
};
use lumina::{
    compile_pass::CompilePass,
    stage::{
        codegen::llvm::FunctionGenerator,
        lex::Lexer,
        lower_ir::{self as ir, IRCtx},
        parse::parse,
    },
    util::source::Source,
};

fn main() {
    let source = Source::new(
        r#"
fn a() -> int {
    return 7;
}

fn main() -> int {
    return a();
}"#,
    );

    let mut ctx = CompilePass::default();

    let program = match parse(&mut ctx, &mut Lexer::new(source)) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let program = match program.ty_solve(&mut ctx) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let main = program.main.name;

    ir::lower(&mut ctx, program);
    let llvm_ctx = inkwell::context::Context::create();
    let module = llvm_ctx.create_module("module");

    let function_map = ctx
        .all_functions()
        .iter()
        .map(|(idx, _)| {
            (
                *idx,
                module.add_function("fn", llvm_ctx.i64_type().fn_type(&[], false), None),
            )
        })
        .collect::<HashMap<_, _>>();

    for (idx, function) in ctx.all_functions() {
        FunctionGenerator::new(
            &mut ctx,
            &llvm_ctx,
            function_map.clone(),
            *function_map.get(&idx).unwrap(),
            function,
        )
        .codegen();
    }

    let main = function_map.get(&main).unwrap();

    // llvm_pass.run_passes(&[
    //     "instcombine",
    //     "reassociate",
    //     "gvn",
    //     "simplifycfg",
    //     "mem2reg",
    // ]);

    module.print_to_stderr();

    let result = jit(&module, *main);
    println!("result: {result}");
}

fn _run_passes(module: &Module, passes: &[&str]) {
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

fn jit(module: &Module, entry: FunctionValue) -> i64 {
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
