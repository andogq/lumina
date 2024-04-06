use crate::util::source::{Span, Spanned};

#[derive(Debug, Clone)]
pub struct Integer {
    pub span: Span,
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self {
            span: Span::default(),
            value,
        }
    }
}

impl Spanned for Integer {
    fn span(&self) -> &Span {
        &self.span
    }
}
