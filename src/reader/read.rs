use super::{super::checker::Checker, super::expr::ExprResult, column::Column, table::Table};

// TODO: Remove Q and T generics once their types are settled
pub struct Reader<'c, C, Q, T> {
    pub(super) checker: &'c mut C,
    pub(super) state: T,
    pub(super) query: Q,
}

pub struct SealedReader<T> {
    state: T,
}

impl<'c, C, Q, T> Reader<'c, C, Q, T> {
    pub fn new(checker: &'c mut C) -> Self
    where
        C: Checker,
        Q: Default,
        T: Default,
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
    pub fn select<'t, 'col, Func, ColIter>(self, _f: Func) -> Result<SealedReader<T>, String>
    where
        Func: FnOnce(Table<'t>) -> ColIter,
        ColIter: IntoIterator<Item = Column<'col>>,
        't: 'col,
    {
        todo!()
    }

    /// Selects all rows for reading, returns a `SealedReader` that cannot be internally modified
    /// further.
    pub fn select_all(self) -> Result<SealedReader<T>, String> {
        todo!()
    }

    /// Filters the rows in the current table
    pub fn filter<F>(self, _f: F) -> Self
    where
        F: FnOnce(Table) -> ExprResult,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{checker::DerivedChecker, expr::funcs::Mappings};
    use super::*; // Required for things to be in scope

    #[test]
    fn reader_syntax() {
        let mut checker = DerivedChecker::new();
        let _x = Reader::<'_, _, usize, usize>::new(&mut checker)
            .table("emp")
            .filter(|t| t["id"].gt(3_usize) & t["name"].len().gt(10_usize))
            .select(|t| [t["id"], t["name"]]);
    }
}
