mod expression;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token encountered")]
    UnexpectedToken(Token),

    #[error("invalid literal, expected `{expected}`")]
    InvalidLiteral { expected: String },
}
