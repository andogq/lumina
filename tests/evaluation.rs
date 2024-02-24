use common::run;
use rust_script::interpreter::return_value::Return;

mod common;

#[test]
fn missing_identifier() {
    let result = run("foobar");

    assert!(matches!(result, Return::Error(_)));

    if let Return::Error(e) = result {
        assert_eq!(e.to_string(), r#"ERROR: identifier not found: "foobar""#);
    }
}
