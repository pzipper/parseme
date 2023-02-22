use super::Peek;

/// An iterator which wraps another iterator and only returns items until the provided condition is
/// true.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NextUntil<'a, Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool> {
    iter: &'a mut Iter,
    cond: Cond,
}

impl<'a, Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool>
    NextUntil<'a, Iter, Cond>
{
    /// Creates a new [NextUntil] from the provided iterator and condition.
    pub fn new(iter: &'a mut Iter, cond: Cond) -> Self {
        Self { iter, cond }
    }
}

impl<'a, Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool> Iterator
    for NextUntil<'a, Iter, Cond>
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

/// Creates an iterator that can be advanced until the provided condition is true.
#[inline]
pub fn next_until<Iter: Iterator + Peek, Cond: FnMut(<Iter as Peek>::Item) -> bool>(
    iter: &mut Iter,
    cond: Cond,
) -> NextUntil<Iter, Cond> {
    NextUntil::new(iter, cond)
}
