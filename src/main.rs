use lumina::compile_and_run;

fn main() {
    let source = r#"
        fn main() -> int {
            let random = 1024;
            let a = [1, 2, 3];

            let i = 0;
            loop {
                if a[i] == 1024 {
                    break;
                }

                i -= 1;
            }

            return a[i];
        }"#;

    let result = compile_and_run(source, true);
    println!("result: {result}");
}
