use inkwell::{module::Module, values::FunctionValue, OptimizationLevel};
use lumina::{
    compile_pass::CompilePass,
    repr::ty::Ty,
    stage::{
        lex::Lexer,
        lower_ir::{self, IRCtx},
        parse::parse,
    },
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
    use std::collections::HashMap;

    use lumina::stage::codegen::llvm::FunctionGenerator;

    let source = Source::new(source);

    let mut ctx = CompilePass::default();

    let program = match parse(&mut ctx, &mut Lexer::new(source)) {
        Ok(output) => output,
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
    let module = llvm_ctx.create_module("module");

    let function_map = ctx
        .all_functions()
        .iter()
        .map(|(idx, f)| {
            (
                *idx,
                module.add_function(
                    "fn",
                    llvm_ctx.i64_type().fn_type(
                        f.signature
                            .arguments
                            .iter()
                            .map(|arg| match arg {
                                Ty::Int => llvm_ctx.i64_type().into(),
                                Ty::Boolean => todo!(),
                                Ty::Unit => todo!(),
                                Ty::Never => {
                                    unreachable!("cannot have function argument that is never type")
                                }
                            })
                            .collect::<Vec<_>>()
                            .as_slice(),
                        false,
                    ),
                    None,
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    for (idx, function) in ctx.all_functions() {
        FunctionGenerator::new(
            &mut ctx,
            &llvm_ctx,
            function_map.clone(),
            *function_map.get(&idx).unwrap(),
            function,
        )
        .codegen();
    }

    let main = function_map.get(&main).unwrap();

    let result = jit(&module, *main);

    assert_eq!(result, expected);
}

fn jit(module: &Module, entry: FunctionValue) -> i64 {
    let engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        engine
            .get_function::<unsafe extern "C" fn() -> i64>(entry.get_name().to_str().unwrap())
            .unwrap()
            .call()
    }
}
