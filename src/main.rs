use lumina::{
    codegen::Compiler,
    core::{ast::parse::parse, lexer::Lexer},
    util::source::Source,
};

fn main() {
    let source = "3 + 4; 8 + 18;";

    let program = parse(Lexer::new(Source::new("hardcoded", source.chars()))).unwrap();

    let compiler = Compiler::new();
    let module = compiler.compile(program);
    module.jit();
}
