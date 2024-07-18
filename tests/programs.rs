use lumina::{
    compile_pass::CompilePass,
    stage::{codegen::llvm::Pass, lex::Lexer, lower_ir, lower_ir::IRCtx, parse::parse},
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

    let mut ctx = CompilePass::default();

    let program = match parse(&mut ctx, &mut Lexer::new(source)) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let program = match program.ty_solve(&mut ctx) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let main = program.main.name;

    lower_ir::lower(&mut ctx, program);
    let llvm_ctx = inkwell::context::Context::create();

    let function_ids = ctx
        .all_functions()
        .iter()
        .map(|(idx, _)| *idx)
        .collect::<Vec<_>>();
    let mut llvm_pass = Pass::new(&llvm_ctx, ctx);
    function_ids.into_iter().for_each(|function| {
        llvm_pass.compile(function);
    });
    let main = *llvm_pass.function_values.get(&main).unwrap();

    let result = llvm_pass.jit(main);

    assert_eq!(result, expected);
}
