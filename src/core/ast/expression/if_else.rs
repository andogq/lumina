use crate::util::source::{Span, Spanned};

use super::{Block, Expression};

#[derive(Debug, Clone)]
pub struct If {
    pub span: Span,
    pub condition: Box<Expression>,
    pub success: Block,
    pub otherwise: Option<Block>,
}

impl If {
    pub fn new(condition: Expression, success: Block, otherwise: Option<Block>) -> Self {
        Self {
            span: Span::default(),
            condition: Box::new(condition),
            success,
            otherwise,
        }
    }
}

impl Spanned for If {
    fn span(&self) -> &Span {
        &self.span
    }
}
