use super::super::prelude::*;

pub struct Column {
    name: String,
    // alias: Option<String>,
}

pub fn col(name: impl Into<String>) -> Column {
    Column {
        name: name.into(),
        // alias: None,
    }
}

impl Expression for Column {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition>> {
        // Conditions:
        // - column exists
        // - column is of the correct type
        Box::new([Condition::ColExistsAndType(self.name.clone(), coerce)].into_iter())
    }

    fn display(&self, dialect: Dialect) -> String {
        self.name.to_string()
    }
}
impl Boolean for Column {}
impl Numeric for Column {}
impl Textual for Column {}
impl Anything for Column {}
