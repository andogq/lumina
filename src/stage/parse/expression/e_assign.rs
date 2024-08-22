pub use super::*;

pub fn parse_assign(compiler: &mut Compiler, tokens: &mut Lexer<'_>) -> Result<Assign, ParseError> {
    let (binding, span_start) = match tokens.next_spanned().unwrap() {
        (Token::Ident(ident), span) => (ident, span),
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Ident(String::new())),
                found: Box::new(token),
                reason: "assign must start with ident".to_string(),
            });
        }
    };

    let binding = compiler.symbols.get_or_intern(binding);

    match tokens.next_token().unwrap() {
        Token::Eq => (),
        token => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Eq),
                found: Box::new(token),
                reason: "equals sign following binding for assign".to_string(),
            });
        }
    }

    let value = parse_expression(compiler, tokens, Precedence::Lowest)?;

    Ok(Assign {
        span: span_start.start..value.span().end,
        binding,
        value: Box::new(value),
        ty_info: None,
    })
}

pub fn parse_op_assign(
    compiler: &mut Compiler,
    tokens: &mut Lexer<'_>,
) -> Result<Assign, ParseError> {
    let (binding, binding_span) = match tokens.next_spanned().unwrap() {
        (Token::Ident(ident), span) => (ident, span),
        (token, _) => {
            return Err(ParseError::ExpectedToken {
                expected: Box::new(Token::Ident(String::new())),
                found: Box::new(token),
                reason: "assign must start with ident".to_string(),
            });
        }
    };

    let binding = compiler.symbols.get_or_intern(binding);

    let operation = match tokens.next_token().unwrap() {
        Token::AddAssign => InfixOperation::Plus,
        Token::MinusAssign => InfixOperation::Minus,
        Token::MulAssign => InfixOperation::Multiply,
        Token::DivAssign => InfixOperation::Divide,
        token => {
            return Err(ParseError::UnexpectedToken(token));
        }
    };

    let right = parse_expression(compiler, tokens, Precedence::Lowest)?;

    Ok(Assign {
        span: binding_span.start..right.span().end,
        binding,
        value: Box::new(Expression::Infix(Infix {
            span: right.span().start..right.span().end,
            left: Box::new(Expression::Ident(Ident {
                span: binding_span,
                binding,
                ty_info: None,
            })),
            operation,
            right: Box::new(right),
            ty_info: None,
        })),
        ty_info: None,
    })
}
