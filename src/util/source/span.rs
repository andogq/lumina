/// A region within a specific file.
pub struct Span {
    pub(super) file_name: String,
    pub(super) start: Location,
    pub(super) end: Location,
}

/// Location of a current line and column.
#[derive(Clone)]
pub struct Location {
    pub(super) line: usize,
    pub(super) column: usize,
}
