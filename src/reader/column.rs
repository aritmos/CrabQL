use super::super::expr::funcs::Mappings;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Column<'c> {
    pub name: &'c str,
}

impl<'c> Column<'c> {
    pub(super) fn new(name: &'c str) -> Self {
        Self { name }
    }
}

impl<'c> Mappings for Column<'c> {}

impl<'c> IntoIterator for Column<'c> {
    type Item = Column<'c>;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}
