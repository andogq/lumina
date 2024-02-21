use std::io::{stdin, stdout, Write};

use crate::lexer::Lexer;

pub fn start() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let Some(Ok(line)) = stdin().lines().next() else {
            break;
        };

        Lexer::new(&line).for_each(|token| {
            println!("{token:?}");
        });
    }
}
