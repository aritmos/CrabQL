use super::Expression;

/// Marker trait for expressions that evaluate into boolean values
pub trait Numeric: Expression {}

impl Expression for i32 {
    fn conditions(
        &self,
        coerce: super::ExprType,
    ) -> Box<dyn Iterator<Item = super::prelude::Condition> + '_> {
        debug_assert!(matches!(
            coerce,
            crate::expr::ExprType::Any | crate::expr::ExprType::Num
        ));

        Box::new(std::iter::empty())
    }

    fn display(&self, dialect: super::Dialect) -> String {
        self.to_string()
    }
}
impl Numeric for i32 {}
