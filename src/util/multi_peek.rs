use std::collections::VecDeque;

/// An iterator that can be peeked ahead multiple times. Similar to [`std::iter::Peekable`].
pub struct MultiPeekable<I>
where
    I: Iterator,
{
    /// Internal iterator
    iter: I,

    /// Buffer containing peeked items
    buffer: VecDeque<I::Item>,
}

impl<I> MultiPeekable<I>
where
    I: Iterator,
{
    /// Create a new multi peek iterator from the provided iterator.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            buffer: VecDeque::new(),
        }
    }

    /// Peek ahead into the iterator by `offset` items. If `offset` is 0, then `None` will be
    /// returned as it is not possible to re-emit an item that has already been emitted.
    pub fn peek_ahead(&mut self, offset: usize) -> Option<&I::Item> {
        // Cannot peek ahead 0 items, as that would be the item that was just emitted
        if offset == 0 {
            return None;
        }

        // Fill the buffer until the offset is satisfied
        while self.buffer.len() < offset {
            self.buffer.push_back(self.iter.next()?);
        }

        self.buffer.get(offset - 1)
    }

    /// Peek ahead a single item in the iterator
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_ahead(1)
    }
}

impl<I> Iterator for MultiPeekable<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front().or_else(|| self.iter.next())
    }
}

pub trait MultiPeekIterator {
    fn multi_peekable(self) -> MultiPeekable<Self>
    where
        Self: Sized + Iterator;
}

impl<I: Iterator> MultiPeekIterator for I {
    fn multi_peekable(self) -> MultiPeekable<Self>
    where
        Self: Sized,
    {
        MultiPeekable::new(self)
    }
}
