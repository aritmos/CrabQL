#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Column<'c> {
    pub(super) name: &'c str,
}

impl<'c> Column<'c> {
    pub(super) fn new(name: &'c str) -> Self {
        Self { name }
    }
}
