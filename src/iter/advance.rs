use super::Peek;

/// Advances the provided iterator while the provided condition is true.
pub fn advance_while<Item>(
    iter: &mut (impl Iterator + Peek<Item = Item>),
    mut cond: impl FnMut(Item) -> bool,
) {
    while let Some(item) = iter.peek() {
        if !cond(item) {
            break;
        }
        iter.next();
    }
}

/// Advances the provided iterator until the provided condition is true.
///
/// Equivalent to `advance_while` with the condition negated.
pub fn advance_until<Item>(
    iter: &mut (impl Iterator + Peek<Item = Item>),
    mut cond: impl FnMut(Item) -> bool,
) {
    while let Some(item) = iter.peek() {
        if cond(item) {
            break;
        }
        iter.next();
    }
}

/// Advances the iterator once, if the provided condition is true.
pub fn next_if<Item>(
    iter: &mut (impl Iterator<Item = Item> + Peek<Item = Item>),
    mut cond: impl FnMut(Item) -> bool,
) -> Option<Item> {
    if let Some(item) = iter.peek() {
        if cond(item) {
            return iter.next();
        }
    }

    None
}
