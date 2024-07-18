use lumina::{
    compile_pass::CompilePass,
    stage::{
        codegen::llvm::Pass,
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

    let function_ids = ctx
        .all_functions()
        .iter()
        .map(|(idx, _)| *idx)
        .collect::<Vec<_>>();
    let mut llvm_pass = Pass::new(&llvm_ctx, ctx);
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
