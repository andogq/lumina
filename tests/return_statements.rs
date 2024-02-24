use rust_script::{
    ast::AstNode,
    interpreter::{
        object::{IntegerObject, Object},
        return_value::Return,
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn return_in_nested_if() {
    let program = r#"
    if (10 > 1) {
        if (10 > 1) {
            return 10;
        }

        return 1;
    }
    "#;

    let result = Parser::new(Lexer::new(program)).parse_program().evaluate();

    assert!(matches!(
        result,
        Return::Explicit(Object::Integer(IntegerObject { value: 10 }))
    ));
}
