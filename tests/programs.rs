use lumina::compile_and_run;
use rstest::rstest;

#[rstest]
#[case::return_constant(
    5,
    r#"
fn main() -> int {
    return 5;
}"#
)]
#[case::return_constant_expression(
    12,
    r#"
fn main() -> int {
    return 5 + 7;
}"#
)]
#[case::variable(
    4,
    r#"
fn main() -> int {
    let a = 4;
    return a;
}"#
)]
#[case::multi_variable_addition(
    17,
    r#"
fn main() -> int {
    let a = 8;
    let b = 9;
    return a + b;
}"#
)]
#[case::block(
    82,
    r#"
fn main() -> int {
    return {
        82
    };
}"#
)]
#[case::block_with_statements(
    82,
    r#"
fn main() -> int {
    return {
        99;
        99;
        82
    };
}"#
)]
#[case::conditional_true(
    27,
    r#"
fn main() -> int {
    if 9 == 9 {
        return 27;
    } else {
        return 3;
    }
}"#
)]
#[case::conditional_false(
    3,
    r#"
fn main() -> int {
    if 5 == 7 {
        return 8;
    } else {
        return 3;
    }
}"#
)]
#[case::condition_in_variable(
    10,
    r#"
fn main() -> int {
    let answer = 1 + 1 == 2;

    let result = if answer {
        10
    } else {
        20
    };

    return result;
}"#
)]
#[case::fibonacci(
    4181,
    r#"fn fib(n: int) -> int {
        if n == 0 {
            return n;
        }

        if n == 1 {
            return n;
        }

        return fib(n - 1) + fib(n - 2);
    }

    fn main() -> int {
        return fib(19);
    }"#
)]
fn programs(#[case] expected: i64, #[case] source: &'static str) {
    let result = compile_and_run(source, false);

    assert_eq!(result, expected);
}
