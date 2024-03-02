use rust_script::{
    lexer::{Lexer, Source},
    parser::Parser,
    runtime::object::Object,
    stages::interpreter::{interpret, runtime::return_value::Return},
};

pub fn run(input: impl AsRef<str>) -> Return<Object> {
    interpret(Parser::new(Lexer::new(Source::new("test", input.as_ref().chars()))).parse_program())
}
