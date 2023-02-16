//! Unicode XID support for Parseme.

use unicode_xid::UnicodeXID;

/// Returns `true` if the provided character is a Unicode XID starting character.
#[inline]
pub fn is_start(char: char) -> bool {
    char.is_xid_start()
}

/// Returns `true` if the provided character is a Unicode XID continuing character.
#[inline]
pub fn is_continue(char: char) -> bool {
    char.is_xid_continue()
}
