use std::{
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use rust_script::{
    core::{
        ast::Program,
        lexer::{Lexer, Source},
        parser::Parser,
    },
    runtime::{object::Object, vm::VM, Environment},
    stages::{
        compiler::Compiler,
        interpreter::{interpret_with_env, runtime::return_value::Return},
    },
};

#[derive(clap::Parser)]
struct Args {
    /// Run with the compiler.
    #[arg(short, long, group = "mode")]
    compiler: bool,

    /// Run with the interpreter.
    #[arg(short, long, group = "mode")]
    interpreter: bool,

    /// Path to a script to run. If not present, a REPL will be started.
    script: Option<PathBuf>,
}

fn main() {
    use clap::Parser;
    let args = Args::parse();

    let mut repl = if args.interpreter {
        Box::new(Interpreted::default())
    } else {
        // Compiled by default
        Box::new(Compiled) as Box<dyn REPL>
    };

    println!("Running in {} mode", repl.get_identifier());

    repl.run();
}

trait REPL {
    fn get_identifier(&self) -> String;
    fn process_program(&mut self, program: Program) -> Result<Option<Object>, String>;

    fn run(&mut self) {
        loop {
            print!(">> ");
            stdout().flush().unwrap();

            let Some(Ok(line)) = stdin().lines().next() else {
                break;
            };

            let lexer = Lexer::new(Source::new("repl", line.chars()));
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();

            if parser.errors.is_empty() {
                match self.process_program(program) {
                    Ok(value) => println!("{value:?}",),
                    Err(err) => eprintln!("{err}"),
                }
            } else {
                parser
                    .errors
                    .iter()
                    .for_each(|err| println!("Error encountered: {err}"));
            }
        }
    }
}

struct Compiled;
impl REPL for Compiled {
    fn get_identifier(&self) -> String {
        "compiled".to_string()
    }

    fn process_program(&mut self, program: Program) -> Result<Option<Object>, String> {
        let mut vm = VM::new(Compiler::compile(program)?);

        vm.run()?;

        Ok(vm.last_pop().cloned())
    }
}

#[derive(Default)]
struct Interpreted {
    env: Environment,
}
impl REPL for Interpreted {
    fn get_identifier(&self) -> String {
        "interpreted".to_string()
    }

    fn process_program(&mut self, program: Program) -> Result<Option<Object>, String> {
        match interpret_with_env(&mut self.env, program) {
            Return::Explicit(value) | Return::Implicit(value) => Ok(Some(value)),
            Return::Error(err) => Err(err.to_string()),
        }
    }
}
