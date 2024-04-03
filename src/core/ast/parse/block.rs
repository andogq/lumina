use crate::core::{
    ast::{source, Block},
    lexer::{token::Token, Lexer},
};

use super::{statement::parse_statement, ParseError};

pub fn parse_block<S>(lexer: &mut Lexer<S>) -> Result<Block, ParseError>
where
    S: Iterator<Item = char>,
{
    lexer.next();

    let block = source::Block {
        statements: std::iter::from_fn(|| {
            if !matches!(lexer.peek(), Token::RightBrace(_)) {
                Some(parse_statement(lexer))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()?,
    };

    // Consume the right brace that just stopped us
    lexer.next();

    Ok(block)
}
