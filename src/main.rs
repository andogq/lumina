use lumina::{
    codegen::{ir, llvm::Pass},
    core::{lexer::Lexer, parse::parse},
    util::source::Source,
};

fn main() {
    let source = Source::new(
        r#"
fn main() -> int {
    let answer = 1 + 1 == 2;

    let result = if answer {
        10
    } else {
        20
    };

    return result;
}"#,
    );

    let program = match parse(Lexer::new(source)) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let program = match program.ty_solve() {
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

    let mut llvm_pass = Pass::new(&ctx, ir_ctx);
    let main = llvm_pass.compile(main);

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
