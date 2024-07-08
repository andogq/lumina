use std::{cell::RefCell, rc::Rc};

use lumina::{
    codegen::{ir, llvm::Pass},
    core::{ctx::Ctx, lexer::Lexer, parse::parse},
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

    let ctx = Ctx::default();

    let (program, ctx) = match parse(ctx, Lexer::new(source)) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let program = match program.ty_solve(Rc::new(RefCell::new(ctx))) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut ir_ctx = ir::lower(program);
    let main = ir_ctx
        .function_for_name("main")
        .expect("main function to exist");

    let ctx = inkwell::context::Context::create();

    let main = ir_ctx.function_for_name("main").unwrap();
    let function_ids = ir_ctx
        .functions
        .iter_enumerated()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    let mut llvm_pass = Pass::new(&ctx, ir_ctx);
    function_ids.into_iter().for_each(|function| {
        llvm_pass.compile(function);
    });
    let main = *llvm_pass.function_values.get(&main).unwrap();

    // llvm_pass.run_passes(&[
    //     "instcombine",
    //     "reassociate",
    //     "gvn",
    //     "simplifycfg",
    //     "mem2reg",
    // ]);

    llvm_pass.debug_print();

    let result = llvm_pass.jit(main);
    println!("result: {result}");

    // let compiler = Compiler::new();
    // let module = compiler.compile(program);
    // module.jit();
}
