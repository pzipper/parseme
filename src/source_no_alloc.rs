use core::iter::Peekable;
use core::str::Chars;

use crate::iter::Peek;

/// An iterator through the characters in a string.
pub struct Source<'a> {
    iter: Peekable<Chars<'a>>,
    src: &'a str,
    pos: usize,
}

impl<'a> Source<'a> {
    /// Creates a new [Source] from the provided string.
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.chars().peekable(),
            src,
            pos: 0,
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
        self.iter.peek().copied()
    }
}

impl<'a> Iterator for Source<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|c| {
            self.pos += c.len_utf8();
            c
        })
    }
}

impl<'a> Peek for Source<'a> {
    type Item = char;

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        self.peek()
    }
}
