use lumina::compile_and_run;

fn main() {
    let source = r#"
        fn fib1(n: int) -> int {
            let a = 0;
            let b = 1;

            let count = 0;

            loop {
                if count == n {
                    break;
                }

                count = count + 1;

                let temp = a;
                a = b;
                b = b + temp;
            }

            return a;
        }

        fn fib2(n: int) -> int {
            if n == 0 || n == 1 {
                return n;
            }

            return fib2(n - 1) + fib2(n - 2);
        }

        fn main() -> int {
            let result1 = fib1(19);
            let result2 = fib2(19);

            if result1 == result2 {
                return result1;
            } else {
                return 0;
            }
        }"#;

    let result = compile_and_run(source, true);
    println!("result: {result}");
}
