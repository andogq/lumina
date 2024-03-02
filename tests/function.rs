use common::run;
use rust_script::{
    runtime::object::{IntegerObject, Object},
    stages::interpreter::runtime::return_value::Return,
};

mod common;

#[test]
fn call_identity_implicit() {
    assert!(matches!(
        run("let identity = fn(x) { x; }; identity(5);"),
        Return::Implicit(Object::Integer(IntegerObject { value: 5 }))
    ));
}

#[test]
fn call_identity_explicit() {
    assert!(matches!(
        run("let identity = fn(x) { return x; }; identity(5);"),
        Return::Implicit(Object::Integer(IntegerObject { value: 5 }))
    ));
}

#[test]
fn call_evaluate_expression() {
    assert!(matches!(
        run("let double = fn(x) { return x * 2; }; double(5);"),
        Return::Implicit(Object::Integer(IntegerObject { value: 10 }))
    ));
}

#[test]
fn call_multiple_arguments() {
    assert!(matches!(
        run("let add = fn(x, y) { return x + y; }; add(5, 6);"),
        Return::Implicit(Object::Integer(IntegerObject { value: 11 }))
    ));
}

#[test]
fn call_nested() {
    assert!(matches!(
        run("let add = fn(x, y) { return x + y; }; add(5 + 5, add(6, 3));"),
        Return::Implicit(Object::Integer(IntegerObject { value: 19 }))
    ));
}

#[test]
fn call_inline() {
    assert!(matches!(
        run("fn(x) { x + 1; }(5)"),
        Return::Implicit(Object::Integer(IntegerObject { value: 6 }))
    ));
}
