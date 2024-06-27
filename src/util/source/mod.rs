mod span;

use std::str::Chars;

pub use self::span::*;

pub struct Source {
    chars: Chars<'static>,

    next_location: Location,
    next_char: Option<char>,
}

impl Source {
    pub fn new(source: &'static str) -> Self {
        Self {
            chars: source.chars(),

            next_location: Location::default(),
            next_char: None,
        }
    }

    /// Emit a location for the current point in the file.
    pub fn location(&self) -> Location {
        self.next_location.clone()
    }

    /// Consume the source whilst the closure is true, returning a string.
    pub fn consume_while(&mut self, mut condition: impl FnMut(&char) -> bool) -> (String, Span) {
        let (str, locations): (String, Vec<_>) = std::iter::from_fn(|| {
            if self.peek().filter(|c| condition(c)).is_some() {
                self.next()
            } else {
                None
            }
        })
        .unzip();

        (
            str,
            (
                // WARN: May misreport on zero-sized strings
                locations.first().unwrap_or(&self.next_location).clone(),
                locations.last().unwrap_or(&self.next_location).clone(),
            )
                .into(),
        )
    }

    pub fn peek(&mut self) -> Option<char> {
        if let Some(c) = self.next_char {
            Some(c)
        } else {
            self.next_char = self.chars.next();
            self.next_char
        }
    }
}

impl Iterator for Source {
    type Item = (char, Location);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.next_char.take().or_else(|| self.chars.next())?;
        let location = self.next_location.clone();

        if c == '\n' {
            self.next_location.next_line();
        } else {
            self.next_location.next();
        }

        Some((c, location))
    }
}
