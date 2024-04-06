/// A region within a specific file.
#[derive(Clone, Debug, Default)]
pub struct Span {
    pub(super) file_name: String,
    pub(super) start: Location,
    pub(super) end: Location,
}

impl Span {
    pub fn to(&self, end: &impl Spanned) -> Self {
        Self {
            file_name: self.file_name.clone(),
            start: self.start.clone(),
            end: end.span().end.clone(),
        }
    }
}

pub trait Spanned {
    fn span(&self) -> &Span;
}

/// Location of a current line and column.
#[derive(Clone, Debug, Default)]
pub struct Location {
    pub(super) line: usize,
    pub(super) column: usize,
}
