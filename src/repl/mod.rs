use std::io::{stdin, stdout, Write};

use crate::{
    core::{
        lexer::{Lexer, Source},
        parser::Parser,
    },
    runtime::vm::VM,
    stages::compiler::Compiler,
};

pub fn start() {
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

            match Compiler::compile(program) {
                Ok(bytecode) => {
                    let mut vm = VM::new(bytecode);

                    if let Err(err) = vm.run() {
                        println!("unable to run program: {err}");
                        continue;
                    }

                    println!("{:?}", vm.last_pop());
                }
                Err(err) => {
                    println!("unable to compile program: {err}");
                }
            }
        }
    }
}
