use inkwell::{
    passes::PassBuilderOptions,
    targets::{CodeModel, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use lumina::{
    codegen::{
        ir::{lowering::lower_function, Context},
        llvm,
    },
    core::{lexer::Lexer, parse::parse},
    util::source::Source,
};

fn main() {
    let source = Source::new(
        r#"
fn main() -> int {
    return 10;
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
    let module = ctx.create_module("main");

    let main = llvm::compile(&ctx, &module, ir, ir_ctx.into_inner().basic_blocks);

    // TODO: Put passes in a different place
    {
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

        module
            .run_passes(
                passes.join(",").as_str(),
                &target_machine,
                PassBuilderOptions::create(),
            )
            .unwrap();
    }

    module.print_to_stderr();

    // TODO: Put JIT in another place
    {
        let engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let result = unsafe {
            engine
                .get_function::<unsafe extern "C" fn() -> i64>(main.get_name().to_str().unwrap())
                .unwrap()
                .call()
        };

        println!("returned value: {result}");
    }

    // let compiler = Compiler::new();
    // let module = compiler.compile(program);
    // module.jit();
}
