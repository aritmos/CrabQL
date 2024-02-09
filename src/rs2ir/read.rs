use super::{column::Column, expr::ExprResult, schema::Schema, table::Table};

pub type ReaderT<'s, S> = Reader<'s, S, usize, Result<(), String>>;

pub struct Reader<'s, S, I, T> {
    pub schema: &'s mut S, // impl Schema
    pub internals: I,
    pub state: T, // Include context (?)
}

pub struct SealedReader<I> {
    _internals: I,
}

impl<'s, S, I, T> Reader<'s, S, I, T> {
    pub fn new(schema: &'s mut S, internals: I, state: T) -> Self
    where
        S: Schema,
    {
        Reader {
            internals,
            schema,
            state,
        }
    }

    /// Adds a table into its context
    pub fn table(self, _id: &str) -> Self {
        todo!()
    }

    /// Adds a group of joined tables into its context
    pub fn tables<'t, Func, TableIter>(self, _f: Func) -> Self
    where
        // RFC: Change this to be an `Expr<Join>` to refuse bad expressions?
        Func: FnOnce(I) -> ExprResult,
        TableIter: IntoIterator<Item = Table<'t>>,
    {
        todo!()
    }

    /// Selects the given rows for reading, returns a `SealedReader` that cannot be internally
    /// modified further.
    pub fn select<'t, 'c, Func, ColIter>(self, _f: Func) -> Result<SealedReader<I>, String>
    where
        Func: FnOnce(Table<'t>) -> ColIter,
        ColIter: IntoIterator<Item = Column<'c>>,
        't: 'c,
    {
        todo!()
    }

    /// Selects all rows for reading, returns a `SealedReader` that cannot be internally modified
    /// further.
    pub fn select_all(self) -> Result<SealedReader<I>, String> {
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
    use super::super::{funcs::Mappings, schema::DerivedSchema};
    use super::*; // Required for things to be in scope

    #[test]
    fn reader_syntax() {
        let mut schema = DerivedSchema::new();
        let reader = schema.read();
        let _x = reader
            .table("emp")
            .filter(|t| t["id"].gt(3_usize) & t["name"].len().gt(10_usize))
            .select(|t| [t["id"], t["name"]]);
    }
}
