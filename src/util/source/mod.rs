mod span;

use std::ops::{Deref, DerefMut};

pub use self::span::*;

use super::multi_peek::{MultiPeekIterator, MultiPeekable};

pub struct Source<S>
where
    S: Iterator,
{
    name: String,

    source: MultiPeekable<S>,
    location: Location,
}

impl<S> Source<S>
where
    S: Iterator<Item = char>,
{
    pub fn new(name: impl ToString, source: impl IntoIterator<Item = char, IntoIter = S>) -> Self {
        Self {
            name: name.to_string(),

            source: source.into_iter().multi_peekable(),
            location: Location { line: 1, column: 0 },
        }
    }

    /// Emit a location for the current point in the file.
    pub fn location(&self) -> Location {
        self.location.clone()
    }

    /// Emit a span starting from the provided location to the current location in the file.
    pub fn span(&self, start: Location) -> Span {
        Span {
            file_name: self.name.clone(),
            start,
            end: self.location(),
        }
    }

    /// Consume the source whilst the closure is true, returning a string.
    pub fn consume_while(&mut self, mut condition: impl FnMut(&char) -> bool) -> String {
        std::iter::from_fn(|| {
            if self.source.peek().filter(|c| condition(c)).is_some() {
                self.next()
            } else {
                None
            }
        })
        .collect()
    }
}

impl<S> Iterator for Source<S>
where
    S: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.source.next()?;

        if c == '\n' {
            self.location.line += 1;
            self.location.column = 0;
        } else {
            self.location.column += 1;
        }

        Some(c)
    }
}

impl<S> Deref for Source<S>
where
    S: Iterator<Item = char>,
{
    type Target = MultiPeekable<S>;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<S> DerefMut for Source<S>
where
    S: Iterator<Item = char>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}
