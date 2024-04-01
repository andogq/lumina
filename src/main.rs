use lumina::{
    codegen::Compiler,
    core::{ast::parse::parse, lexer::Lexer},
    util::source::Source,
};

fn main() {
    let source = "fn main() -> int { 1 + 2; return 26 + 104 + 11; }";

    let program = parse(Lexer::new(Source::new("hardcoded", source.chars()))).unwrap();

    let compiler = Compiler::new();
    let module = compiler.compile(program);
    module.jit();
}
