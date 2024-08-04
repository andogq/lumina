use lumina::compile_and_run;

fn main() {
    let source = r#"
        fn fib(n: int) -> int {
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
        }"#;

    let result = compile_and_run(source, true);
    println!("result: {result}");
}
