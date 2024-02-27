use std::io::{stdin, stdout, Write};

use crate::{
    compiler::compile,
    interpreter::environment::Environment,
    lexer::{Lexer, Source},
    parser::Parser,
    vm::VM,
};

pub fn start() {
    let env = Environment::new();

    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let Some(Ok(line)) = stdin().lines().next() else {
            break;
        };

        let lexer = Lexer::new(Source::new("repl", line.chars()));
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        parser
            .errors
            .iter()
            .for_each(|err| println!("Error encountered: {err}"));

        if parser.errors.is_empty() {
            // println!("{}", program.evaluate(env.clone()));

            match compile(program) {
                Ok(bytecode) => {
                    let mut vm = VM::new(bytecode);

                    if let Err(err) = vm.run() {
                        println!("unable to run program: {err}");
                        continue;
                    }

                    println!("stack top: {:?}", vm.stack_top());
                }
                Err(err) => {
                    println!("unable to compile program: {err}");
                }
            }
        }
    }
}
