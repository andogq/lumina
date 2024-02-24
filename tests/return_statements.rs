mod common;

use rust_script::interpreter::{
    object::{IntegerObject, Object},
    return_value::Return,
};

use crate::common::run;

#[test]
fn return_in_nested_if() {
    let result = run(r#"
    if (10 > 1) {
        if (10 > 1) {
            return 10;
        }

        return 1;
    }
    "#);

    assert!(matches!(
        result,
        Return::Explicit(Object::Integer(IntegerObject { value: 10 }))
    ));
}
