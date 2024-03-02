use std::collections::VecDeque;

use crate::core::lexer::span::{Location, Span};

pub struct Source<S> {
    name: String,

    source: S,
    line: usize,
    column: usize,

    buffer: VecDeque<char>,
}

impl<S> Source<S>
where
    S: Iterator<Item = char>,
{
    pub fn new<I>(name: impl ToString, source: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = S>,
    {
        Self {
            name: name.to_string(),

            source: source.into_iter(),
            line: 1,
            column: 0,

            buffer: VecDeque::new(),
        }
    }

    pub fn location(&self) -> Location {
        Location::new(self.line, self.column)
    }

    pub fn span(&self, start: Location) -> Span {
        Span::new(&self.name, start, self.location())
    }

    pub fn peek_ahead(&mut self, offset: usize) -> Option<char> {
        while self.buffer.len() < offset {
            self.buffer.push_back(self.source.next()?);
        }

        self.buffer.get(offset.checked_sub(1)?).cloned()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.peek_ahead(1)
    }

    pub fn consume_while(&mut self, mut condition: impl FnMut(char) -> bool) -> String {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if !condition(c) {
                break;
            }

            self.next();
            s.push(c);
        }

        s
    }
}

impl<S> Iterator for Source<S>
where
    S: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Either take something out of the buffer, or the iterator
        let c = self.buffer.pop_front().or_else(|| self.source.next())?;

        if c == '\n' {
            // New line encountered, reset column
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(c)
    }
}
