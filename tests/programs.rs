use lumina::{
    codegen::{ir, llvm::Pass},
    core::{ctx::Ctx, lexer::Lexer, parse::parse},
    util::source::Source,
};
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
fn programs(#[case] expected: i64, #[case] source: &'static str) {
    let source = Source::new(source);

    let program = match parse(Ctx::default(), Lexer::new(source)) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    }
    .0;

    let program = match program.ty_solve() {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut ir_ctx = ir::lower(program);
    let main = ir_ctx
        .function_for_name("main")
        .expect("main function to exist");

    let ctx = inkwell::context::Context::create();

    let mut llvm_pass = Pass::new(&ctx, ir_ctx);
    let main = llvm_pass.compile(main);

    let result = llvm_pass.jit(main);

    assert_eq!(result, expected);
}
