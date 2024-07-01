use crate::core::{ast::*, lexer::Lexer, parse::ParseError, symbol::SymbolMap};

pub fn parse_ident(lexer: &mut Lexer, symbols: &mut SymbolMap) -> Result<Ident, ParseError> {
    let token = lexer.ident("ident peeked")?;

    Ok(Ident {
        span: token.span,
        name: symbols.get(token.literal),
    })
}

#[cfg(test)]
mod test {
    use super::{parse_ident, *};
    use crate::core::lexer::token::Token;

    use rstest::rstest;

    #[test]
    fn success() {
        let mut lexer = Lexer::with_tokens(vec![Token::ident("someident")]);
        let mut symbols = SymbolMap::new();

        let ident = parse_ident(&mut lexer, &mut symbols).unwrap();
        assert_eq!(symbols.name(ident.name).unwrap(), "someident");
    }

    #[test]
    fn fail() {
        let mut lexer = Lexer::with_tokens(vec![Token::integer("1")]);
        let mut symbols = SymbolMap::new();

        assert!(parse_ident(&mut lexer, &mut symbols).is_err());
    }

    #[rstest]
    #[case::success(Token::ident("someident"))]
    #[case::fail(Token::integer("1"))]
    fn single_token(#[case] token: Token) {
        let mut lexer = Lexer::with_tokens(vec![token, Token::semicolon()]);
        let mut symbols = SymbolMap::new();
        let _ = parse_ident(&mut lexer, &mut symbols);

        assert_eq!(lexer.into_iter().count(), 1);
    }
}
