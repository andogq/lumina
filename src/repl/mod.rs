use std::io::{stdin, stdout, Write};

use crate::{lexer::Lexer, token::Token};

pub fn start() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let Some(Ok(line)) = stdin().lines().next() else {
            break;
        };

        let mut lexer = Lexer::new(&line);
        loop {
            let token = lexer.next_token();
            println!("{token:?}");

            if matches!(token, Token::EOF) {
                break;
            }
        }
    }
}
