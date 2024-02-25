use rust_script::{
    ast::AstNode,
    interpreter::{environment::Environment, object::Object, return_value::Return},
    lexer::{Lexer, Source},
    parser::Parser,
};

pub fn run(input: impl AsRef<str>) -> Return<Object> {
    Parser::new(Lexer::new(Source::new("test", input.as_ref().chars())))
        .parse_program()
        .evaluate(Environment::new())
}
