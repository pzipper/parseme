use super::Peek;

/// An iterator which wraps another iterator and only returns items while the provided condition is
/// true.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NextWhile<'a, Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool> {
    iter: &'a mut Iter,
    cond: Cond,
}

impl<'a, Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool>
    NextWhile<'a, Iter, Cond>
{
    /// Creates a new [NextWhile] from the provided iterator and condition.
    pub fn new(iter: &'a mut Iter, cond: Cond) -> Self {
        Self { iter, cond }
    }
}

impl<'a, Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool> Iterator
    for NextWhile<'a, Iter, Cond>
{
    type Item = <Iter as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.peek() {
            if (self.cond)(item) {
                return self.iter.next();
            }
        }

        None
    }
}

/// Creates an iterator that can be advanced until the provided condition is false.
#[inline]
pub fn next_while<Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool>(
    iter: &mut Iter,
    cond: Cond,
) -> NextWhile<Iter, Cond> {
    NextWhile::new(iter, cond)
}
