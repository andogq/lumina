use crate::{
    core::{
        ast::Block,
        lexer::{token::Token, Lexer},
        symbol::SymbolMap,
    },
    util::source::Spanned,
};

use super::{statement::parse_statement, ParseError};

pub fn parse_block<S>(lexer: &mut Lexer<S>, symbols: &mut SymbolMap) -> Result<Block, ParseError>
where
    S: Iterator<Item = char>,
{
    let Token::LeftBrace(open_brace) = lexer.next() else {
        return Err(ParseError::ExpectedToken("{".to_string()));
    };

    let statements = std::iter::from_fn(|| {
        if !matches!(lexer.peek(), Token::RightBrace(_)) {
            Some(parse_statement(lexer, symbols))
        } else {
            None
        }
    })
    .collect::<Result<Vec<_>, _>>()?;

    // Consume the right brace that just stopped us
    let Token::RightBrace(close_brace) = lexer.next() else {
        return Err(ParseError::ExpectedToken("}".to_string()));
    };

    Ok(Block {
        statements,
        span: open_brace.span().to(&close_brace),
    })
}
