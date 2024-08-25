use lumina::compile_and_run;

fn main() {
    let source = r#"
        fn main() -> int {
            let a = [1, 2, 3];

            let b = a[1];

            return a[2];
        }"#;

    let result = compile_and_run(source, true);
    println!("result: {result}");
}
