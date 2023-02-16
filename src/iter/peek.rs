/// An iterator or stream which implements peeking.
pub trait Peek {
    /// The type of item produced by the iterator or stream.
    type Item;

    /// Returns the next item in the iterator or stream without advancing it.
    fn peek(&mut self) -> Option<Self::Item>;
}
