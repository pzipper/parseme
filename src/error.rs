/// An error type for a parser.
///
/// TODO: make derive macro for this
pub trait Error {
    /// Returns `true` if this [Error] represents failing to find a match.
    fn is_no_match(&self) -> bool;
}

/// A generic error representing no match.
///
/// Used if the only possible error is a no match error.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoMatchError;

impl Error for NoMatchError {
    #[inline]
    fn is_no_match(&self) -> bool {
        true
    }
}
