//! Iterators which may fail.

/// An iterator which may fail.
///
/// Similar to the [Iterator] trait, but uses [Result] instead of [Option].  Generally, [Stream]s
/// are used for [Iterator]s which calculate values on demand, such as a lexer or parser.
pub trait Stream {
    /// The type of the items returned by the [Stream] if no error occurs.
    type Item;

    /// The type of error that may occur while iterating.
    type Error;

    /// Returns the next item in the stream, or an error if one occurs.
    fn next(&mut self) -> Result<Self::Item, Self::Error>;
}
