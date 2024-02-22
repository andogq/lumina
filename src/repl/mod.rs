use std::io::{stdin, stdout, Write};

use crate::{lexer::Lexer, parser::Parser};

pub fn start() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let Some(Ok(line)) = stdin().lines().next() else {
            break;
        };

        let lexer = Lexer::new(&line);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        parser
            .errors
            .iter()
            .for_each(|err| println!("Error encountered: {err}"));

        println!("{}", program.to_string());
    }
}
