use crate::{
    core::ast::Statement,
    util::source::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            span: Span::default(),
            statements: statements.to_vec(),
        }
    }
}

impl Spanned for Block {
    fn span(&self) -> &Span {
        &self.span
    }
}
