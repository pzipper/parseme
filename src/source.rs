extern crate alloc;

use alloc::collections::VecDeque;
use core::str::Chars;

use crate::iter::Peek;

/// An iterator through the characters in a string.
pub struct Source<'a> {
    iter: Chars<'a>,
    src: &'a str,
    pos: usize,
    peeked: VecDeque<char>,
}

impl<'a> Source<'a> {
    /// Creates a new [Source] from the provided string.
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.chars(),
            src,
            pos: 0,
            peeked: VecDeque::new(),
        }
    }

    /// Returns the current position offset (in bytes) from the start of the source string.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the source string which is being iterated through.
    #[inline]
    pub fn src(&self) -> &'a str {
        self.src
    }

    /// Returns the next character in the iterator without advancing it.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.peek_nth(0)
    }

    /// Returns the *nth* future character in the iterator.
    ///
    /// Uses zero-based indexing (i.e. `0` returns the next character, `1` the character after
    /// that, and so on).
    pub fn peek_nth(&mut self, nth: usize) -> Option<char> {
        self.peeked.extend(
            self.iter
                .by_ref()
                .take((nth + 1).saturating_sub(self.peeked.len())),
        );
        self.peeked.get(nth).copied()
    }
}

impl<'a> Iterator for Source<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.pop_front().or_else(|| self.iter.next()) {
            Some(next) => {
                self.pos += next.len_utf8();
                Some(next)
            }
            None => None,
        }
    }
}

impl<'a> Peek for Source<'a> {
    type Item = char;

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        self.peek()
    }
}
