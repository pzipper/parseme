use crate::{Error, Parser};

/// An error that occurred during the parsing of a [Group].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupError<Err> {
    /// The [Group] failed to find a match.
    NoMatch,

    /// A user-defined error.
    User(Err),
}

impl<Err> Error for GroupError<Err> {
    #[inline]
    fn is_no_match(&self) -> bool {
        matches!(self, Self::NoMatch)
    }
}

/// A group of [Parser]s.
pub struct Group<In: ?Sized, Out, Err: Error> {
    parsers: Vec<Box<dyn Parser<In, Out, Err>>>,
}

impl<In: ?Sized, Out, Err: Error> Group<In, Out, Err> {
    /// Creates a new, empty [Group].
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
        }
    }

    /// Adds a [Parser] to the [Group].
    #[inline]
    pub fn add(mut self, parser: impl Parser<In, Out, Err> + 'static) -> Self {
        self.parsers.push(Box::new(parser));
        self
    }
}

impl<In: ?Sized, Out, Err: Error> Parser<In, Out, GroupError<Err>> for Group<In, Out, Err> {
    #[inline]
    fn parse(&mut self, input: &mut In) -> Result<Out, GroupError<Err>> {
        for parser in &mut self.parsers {
            match parser.parse(input) {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if err.is_no_match() {
                        continue;
                    } else {
                        return Err(GroupError::User(err));
                    }
                }
            }
        }

        Err(GroupError::NoMatch)
    }
}
