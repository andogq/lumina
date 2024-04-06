use crate::util::source::{Span, Spanned};

#[derive(Debug, Clone)]
pub struct Boolean {
    pub span: Span,
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self {
            span: Span::default(),
            value,
        }
    }
}

impl Spanned for Boolean {
    fn span(&self) -> &Span {
        &self.span
    }
}
