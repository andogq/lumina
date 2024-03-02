use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Span {
    source: String,
    start: Location,
    end: Location,
}

impl Span {
    pub fn new(source: impl ToString, start: Location, end: Location) -> Self {
        Self {
            source: source.to_string(),
            start,
            end,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({} - {})", self.source, self.start, self.end)
    }
}
