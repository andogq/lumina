use lumina::compile_and_run;

fn main() {
    let source = r#"
        fn main() -> int {
            let counter = 5;

            loop {
                if counter == 5 {
                    break;
                }

                let a = 3;
            }

            return counter;
        }"#;

    let result = compile_and_run(source, true);
    println!("result: {result}");
}
