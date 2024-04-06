use lumina::{
    codegen::Compiler,
    core::{lexer::Lexer, parse::parse},
    util::source::Source,
};

fn main() {
    let source = r#"
fn main() -> int {
    let a = { 5; 1 + 2 };
    let b = 4;

    let value = true;

    let z = if false {
        4
    } else {
        5
    };

    return b + a + 10 + z;
}"#;

    let program = parse(Lexer::new(Source::new("hardcoded", source.chars()))).unwrap();

    if let Err(e) = program.type_check() {
        eprintln!("{e}");
        return;
    }

    let compiler = Compiler::new();
    let module = compiler.compile(program);
    module.jit();
}
