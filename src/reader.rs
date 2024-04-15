use crate::checker::Checker;
use crate::expr::{bool::Boolean, CommonExpr, Expression};

// The query that is being built
type Q = ();

// The context/state of the reader.
// Used for validating expressions
type T = ();

// TODO: Remove Q and T generics once their types are settled
pub struct Reader<'c, C> {
    pub(super) checker: &'c mut C,
    pub(super) state: T,
    pub(super) query: Q,
}

pub struct SealedReader<T> {
    state: T,
}

impl<'c, C> Reader<'c, C> {
    pub fn new(checker: &'c mut C) -> Self
    where
        C: Checker,
    {
        Reader {
            checker,
            state: T::default(),
            query: Q::default(),
        }
    }

    /// Adds a table into its state
    pub fn table(self, _id: &str) -> Self {
        todo!()
    }

    /// Selects the given rows for reading, returns a `SealedReader` that cannot be internally
    /// modified further.
    pub fn select(self, expr: impl Expression) -> Result<SealedReader<T>, String> {
        todo!()
    }

    /// Selects all rows for reading, returns a `SealedReader` that cannot be internally modified
    /// further.
    pub fn select_all(self) -> Result<SealedReader<T>, String> {
        todo!()
    }

    /// Filters the rows in the current table
    pub fn filter<T: Boolean>(self, bool_expr: CommonExpr<T>) -> Self {
        todo!()
    }
}
