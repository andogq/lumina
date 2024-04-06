use crate::util::source::Span;

#[derive(Debug)]
pub struct Integer {
    pub span: Span,

    pub value: i64,
}
