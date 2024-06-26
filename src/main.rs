use lumina::{
    codegen::{ir::Builder, llvm::Pass},
    core::{lexer::Lexer, parse::parse},
    util::source::Source,
};

fn main() {
    let source = Source::new(
        r#"
fn main() -> int {
    let result = if 2 {
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

    if let Err(e) = program.type_check() {
        eprintln!("{e}");
        return;
    }

    let mut ir_builder = Builder::default();
    let function_id = ir_builder.lower_function(program.main);
    let ir_ctx = ir_builder.consume();

    let ctx = inkwell::context::Context::create();

    let mut llvm_pass = Pass::new(&ctx, ir_ctx);
    let main = llvm_pass.compile(function_id);

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
