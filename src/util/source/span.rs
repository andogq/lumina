use std::fmt::{Debug, Display};

/// A region within a specific file.
#[derive(Clone, Default)]
pub struct Span {
    pub(super) start: Location,
    pub(super) end: Location,
}

impl Span {
    pub fn to(&self, end: &impl Spanned) -> Self {
        Self {
            start: self.start.clone(),
            end: end.span().end.clone(),
        }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{start} -> {end}", start = self.start, end = self.end,)
    }
}

impl From<(Location, Location)> for Span {
    fn from((start, end): (Location, Location)) -> Self {
        Self { start, end }
    }
}

pub trait Spanned {
    fn span(&self) -> &Span;
}

impl Spanned for Span {
    fn span(&self) -> &Span {
        self
    }
}

/// Location of a current line and column.
#[derive(Clone)]
pub struct Location {
    pub(super) line: usize,
    pub(super) column: usize,

    // Offset into a buffer of the source
    pub(super) offset: usize,
}

impl Location {
    pub fn next(&mut self) {
        self.offset += 1;
        self.column += 1;
    }

    pub fn next_line(&mut self) {
        self.offset += 1;
        self.column = 0;
        self.line += 1;
    }

    pub fn span(self) -> Span {
        (self.clone(), self).into()
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 0,
            offset: 0,
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self} ({offset})", offset = self.offset)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{column}", line = self.line, column = self.column)
    }
}
