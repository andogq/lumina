use lumina::{
    codegen::{
        ir::{lowering::lower_function, Context},
        llvm::Pass,
    },
    core::{lexer::Lexer, parse::parse},
    util::source::Source,
};

fn main() {
    let source = Source::new(
        r#"
fn main() -> int {
    let a = 10;
    let b = 7;
    let c = a;
    return c;
}"#,
    );

    let program = match parse(Lexer::new(source)) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    if let Err(e) = program.type_check() {
        eprintln!("{e}");
        return;
    }

    let ir_ctx = Context::new();
    let ir = lower_function(&ir_ctx, program.main);

    let ctx = inkwell::context::Context::create();

    let llvm_pass = Pass::new(&ctx, ir_ctx);
    let main = llvm_pass.compile(ir);

    llvm_pass.run_passes(&[
        "instcombine",
        "reassociate",
        "gvn",
        "simplifycfg",
        "mem2reg",
    ]);

    llvm_pass.debug_print();

    let result = llvm_pass.jit(main);
    println!("result: {result}");

    // let compiler = Compiler::new();
    // let module = compiler.compile(program);
    // module.jit();
}
