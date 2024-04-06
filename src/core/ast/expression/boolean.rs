use crate::util::source::Span;

#[derive(Debug)]
pub struct Boolean {
    pub span: Span,
    pub value: bool,
}
