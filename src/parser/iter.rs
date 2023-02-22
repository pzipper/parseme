// extern crate core;

use core::marker::PhantomData;

use crate::parser::Parser;

/// An [Iterator] which wraps a [Parser] and its input.
///
/// Calling [Iterator::next] will call the [Parser::parse] method of the wrapped [Parser], with the
/// input that the [ParserIter] was initialized with.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParserIter<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>> {
    marker: PhantomData<(Out, Err)>,
    input: &'input mut In,
    parser: &'parser mut P,
}

impl<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>>
    ParserIter<'input, 'parser, In, Out, Err, P>
{
    /// Creates a new [ParserIter] from the provided input and [Parser].
    pub fn new(input: &'input mut In, parser: &'parser mut P) -> Self {
        Self {
            marker: PhantomData,
            input,
            parser,
        }
    }
}

impl<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>> Iterator
    for ParserIter<'input, 'parser, In, Out, Err, P>
{
    type Item = Result<Out, Err>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.parser.parse(self.input))
    }
}
