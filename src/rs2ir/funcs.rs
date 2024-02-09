// the `len()` method does not measure the length of the collection
#[allow(clippy::len_without_is_empty)]
pub trait Functions {
    type Expr;

    /// Returns the length of the value
    fn len(self) -> Self::Expr;
}
