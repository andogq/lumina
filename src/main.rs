use lumina::compile_and_run;

fn main() {
    let source = r#"
        fn condition(n: int) -> bool {
            n == 0 || n == 1
        }

        fn fib(n: int) -> int {
            if condition(n) {
                n
            } else {
                fib(n - 1) + fib(n - 2)
            }
        }

        fn main() -> int {
            fib(19)
        }"#;

    let result = compile_and_run(source, true);
    println!("result: {result}");
}
