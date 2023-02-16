/// An error type for a parser.
///
/// TODO: make derive macro for this
pub trait Error {
    /// Returns `true` if this [Error] represents failing to find a match.
    fn is_no_match(&self) -> bool;
}

impl Error for () {
    #[inline]
    fn is_no_match(&self) -> bool {
        true
    }
}
