// extern crate core;

use core::marker::PhantomData;

use crate::{parser::Parser, stream::Stream};

/// A [Stream] which wraps a [Parser] and its input.
///
/// Calling [Stream::next] will call the [Parser::parse] method of the wrapped [Parser], with the
/// input that the [ParserStream] was initialized with.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParserStream<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>> {
    marker: PhantomData<(Out, Err)>,
    input: &'input mut In,
    parser: &'parser mut P,
}

impl<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>>
    ParserStream<'input, 'parser, In, Out, Err, P>
{
    /// Creates a new [ParserStream] from the provided input and [Parser].
    pub fn new(input: &'input mut In, parser: &'parser mut P) -> Self {
        Self {
            marker: PhantomData,
            input,
            parser,
        }
    }
}

impl<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>> Stream
    for ParserStream<'input, 'parser, In, Out, Err, P>
{
    type Item = Out;
    type Error = Err;

    #[inline]
    fn next(&mut self) -> Result<Self::Item, Self::Error> {
        self.parser.parse(self.input)
    }
}
