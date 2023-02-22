// extern crate core;

use core::marker::PhantomData;

use crate::parser::Parser;

/// An [Iterator] which wraps a [Parser] and its input.
///
/// Calling [Iterator::next] will call the [Parser::parse] method of the wrapped [Parser], with the
/// input that the [ParserIter] was initialized with.  Once an error has been thrown, the
/// [ParserIter] will return continuously [None].
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParserIter<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>> {
    marker: PhantomData<(Out, Err)>,
    input: &'input mut In,
    parser: &'parser mut P,
    error: bool,
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
            error: false,
        }
    }
}

impl<'input, 'parser, In: ?Sized, Out, Err, P: Parser<In, Out, Err>> Iterator
    for ParserIter<'input, 'parser, In, Out, Err, P>
{
    type Item = Result<Out, Err>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        let res = self.parser.parse(self.input);

        match res {
            Ok(value) => Some(Ok(value)),
            Err(err) => {
                self.error = true;
                Some(Err(err))
            }
        }
    }
}
