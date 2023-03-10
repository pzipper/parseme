#![doc = include_str!("../README.md")]
#![no_std]

mod error;
pub mod iter;
pub mod parser;
pub mod stream;

pub use error::*;
pub use parser::Parser;

#[cfg(feature = "alloc")]
mod source;
#[cfg(not(feature = "alloc"))]
#[path = "source_no_alloc.rs"]
mod source;

pub use source::*;

pub mod xid {
    //! Unicode XID support for Parseme.
    pub use parseme_xid::*;
}

/// Convenience method to convert a function into a [Parser] implementation.
///
/// # Examples
/// ```
/// use parseme::Parser;
///
/// fn my_parser(input: &mut [u8]) -> Result<u8, ()> {
///     Ok(*input.first_mut().unwrap())
/// }
///
/// assert_eq!(parseme::new(my_parser).parse(&mut [42, 64]), Ok(42));
/// ```
#[inline]
pub const fn new<In: ?Sized, Out, Err>(
    parser: impl Parser<In, Out, Err>,
) -> impl Parser<In, Out, Err> {
    parser
}
