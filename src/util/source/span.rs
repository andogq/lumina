/// A region within a specific file.
#[derive(Clone, Debug, Default)]
pub struct Span {
    pub(super) file_name: String,
    pub(super) start: Location,
    pub(super) end: Location,
}

/// Location of a current line and column.
#[derive(Clone, Debug, Default)]
pub struct Location {
    pub(super) line: usize,
    pub(super) column: usize,
}
