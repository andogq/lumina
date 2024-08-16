use std::collections::HashMap;

use compiler::Compiler;
use inkwell::{
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    types::BasicType,
    values::FunctionValue,
    OptimizationLevel,
};
use repr::ty::Ty;
use stage::codegen::llvm::FunctionGenerator;

pub mod compiler;
pub mod repr;
pub mod stage;
pub mod util;

pub fn compile_and_run(source: &'static str, debug: bool) -> i64 {
    let mut compiler = Compiler::default();
    let functions = compiler.compile(source).unwrap();

    let llvm_ctx = inkwell::context::Context::create();
    let module = llvm_ctx.create_module("module");

    let function_map = functions
        .iter()
        .map(|function| {
            (
                function.identifier,
                module.add_function(
                    compiler
                        .get_function_symbol(function.identifier)
                        .and_then(|symbol| compiler.get_interned_string(symbol))
                        .expect("function to have name"),
                    {
                        let return_ty = match function.signature.return_ty {
                            Ty::Int => llvm_ctx.i64_type().as_basic_type_enum(),
                            Ty::Boolean => llvm_ctx.bool_type().as_basic_type_enum(),
                            Ty::Unit => todo!(),
                            Ty::Never => todo!(),
                        };

                        return_ty.fn_type(
                            function
                                .signature
                                .arguments
                                .iter()
                                .map(|arg| match arg {
                                    Ty::Int => llvm_ctx.i64_type().into(),
                                    Ty::Boolean => llvm_ctx.bool_type().into(),
                                    Ty::Unit => todo!(),
                                    Ty::Never => {
                                        unreachable!("cannot have an argument that is never type")
                                    }
                                })
                                .collect::<Vec<_>>()
                                .as_slice(),
                            false,
                        )
                    },
                    None,
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    for function in &functions {
        FunctionGenerator::new(
            &mut compiler,
            &llvm_ctx,
            function_map.clone(),
            *function_map.get(&function.identifier).unwrap(),
            function.clone(),
        )
        .codegen();
    }

    let main_symbol = compiler.intern_string("main");
    let main = functions
        .iter()
        .map(|f| f.identifier)
        .find(|identifier| compiler.get_function_symbol(*identifier).unwrap() == main_symbol)
        .unwrap();
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
