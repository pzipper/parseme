//! The [Parser] trait.

#[cfg(feature = "alloc")]
mod group;
mod stream;

#[cfg(feature = "alloc")]
pub use group::*;
pub use stream::*;

/// Any parser compatible with Parseme.
pub trait Parser<In: ?Sized, Out, Err> {
    /// Parses the provided input and returns the result.
    fn parse(&mut self, input: &mut In) -> Result<Out, Err>;

    /// Returns a [ParserStream] which wraps this [Parser] and the provided input.
    #[inline]
    fn stream<'input, 'parser>(
        &'parser mut self,
        input: &'input mut In,
    ) -> ParserStream<'input, 'parser, In, Out, Err, Self>
    where
        Self: Sized,
    {
        ParserStream::new(input, self)
    }
}

impl<In: ?Sized, Out, Err, F: FnMut(&mut In) -> Result<Out, Err>> Parser<In, Out, Err> for F {
    #[inline]
    fn parse(&mut self, input: &mut In) -> Result<Out, Err> {
        self(input)
    }
}
