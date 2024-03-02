use common::run;
use rust_script::{
    runtime::object::{IntegerObject, Object},
    stages::interpreter::runtime::return_value::Return,
};

mod common;

#[test]
fn single_assignment() {
    let result = run("let a = 5; a;");

    assert!(matches!(
        result,
        Return::Implicit(Object::Integer(IntegerObject { value: 5 }))
    ));
}

#[test]
fn single_assignment_expression() {
    let result = run("let a = 5 * 5; a;");

    assert!(matches!(
        result,
        Return::Implicit(Object::Integer(IntegerObject { value: 25 }))
    ));
}

#[test]
fn multi_assignment() {
    let result = run("let a = 5; let b = a; b;");

    assert!(matches!(
        result,
        Return::Implicit(Object::Integer(IntegerObject { value: 5 }))
    ));
}

#[test]
fn multi_assignment_expression() {
    let result = run("let a = 5; let b = a; let c = a + b + 5; c;");

    assert!(matches!(
        result,
        Return::Implicit(Object::Integer(IntegerObject { value: 15 }))
    ));
}
