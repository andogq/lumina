use std::collections::HashMap;

use compile_pass::CompilePass;
use inkwell::{
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    values::FunctionValue,
    OptimizationLevel,
};
use repr::ty::Ty;
use stage::{
    codegen::{ctx::LLVMCtx as _, llvm::FunctionGenerator},
    lex::Lexer,
    lower_ir::{self, IRCtx as _},
    parse::parse,
};
use util::source::Source;

pub mod compile_pass;
pub mod repr;
pub mod stage;
pub mod util;

pub fn compile_and_run(source: &'static str, debug: bool) -> i64 {
    let source = Source::new(source);

    let mut ctx = CompilePass::default();

    let program = parse(&mut ctx, &mut Lexer::new(source)).unwrap();

    let program = program.ty_solve(&mut ctx).unwrap();

    let main = program.main.name;

    lower_ir::lower(&mut ctx, program);
    let llvm_ctx = inkwell::context::Context::create();
    let module = llvm_ctx.create_module("module");

    let function_map = ctx
        .all_functions()
        .iter()
        .map(|(idx, f)| {
            (
                *idx,
                module.add_function(
                    &ctx.get_function_name(idx),
                    llvm_ctx.i64_type().fn_type(
                        f.signature
                            .arguments
                            .iter()
                            .map(|arg| match arg {
                                Ty::Int => llvm_ctx.i64_type().into(),
                                Ty::Boolean => todo!(),
                                Ty::Unit => todo!(),
                                Ty::Never => {
                                    unreachable!("cannot have an argument that is never type")
                                }
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        false,
                    ),
                    None,
                ),
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
fn run_passes(module: &Module, passes: &[&str]) {
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
