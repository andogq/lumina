use rust_script::{
    ast::AstNode,
    interpreter::{environment::Environment, object::Object, return_value::Return},
    lexer::Lexer,
    parser::Parser,
};

pub fn run(input: impl AsRef<str>) -> Return<Object> {
    Parser::new(Lexer::new(input.as_ref()))
        .parse_program()
        .evaluate(&mut Environment::new())
}
