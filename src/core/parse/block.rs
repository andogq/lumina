use crate::{
    core::{
        ast::Block,
        lexer::{
            token::{LeftBraceToken, RightBraceToken, Token},
            Lexer,
        },
        symbol::SymbolMap,
    },
    util::source::Spanned,
};

use super::{statement::parse_statement, ParseError};

pub fn parse_block(lexer: &mut Lexer, symbols: &mut SymbolMap) -> Result<Block, ParseError> {
    let open_brace = match lexer.next_token() {
        Token::LeftBrace(ident) => ident,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::LeftBrace(LeftBraceToken::default())),
                found: Box::new(token),
                reason: "block must begin with an opening brace".to_string(),
            });
        }
    };

    let statements = std::iter::from_fn(|| {
        if !matches!(lexer.peek_token(), Token::RightBrace(_)) {
            Some(parse_statement(lexer, symbols))
        } else {
            None
        }
    })
    .collect::<Result<Vec<_>, _>>()?;

    // Consume the right brace that just stopped us
    let close_brace = match lexer.next_token() {
        Token::RightBrace(ident) => ident,
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::RightBrace(RightBraceToken::default())),
                found: Box::new(token),
                reason: "block must end with a closing brace".to_string(),
            });
        }
    };

    Ok(Block {
        statements,
        span: open_brace.span().to(&close_brace),
    })
}
