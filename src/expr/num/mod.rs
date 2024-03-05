//! Expressions that evaluate into numeric values

use super::{CoreExpression, Expression};

pub mod arith;
#[doc(inline)]
pub use arith::{Add, AddExpr, Div, DivExpr, Mul, MulExpr, Rem, RemExpr, Sub, SubExpr};

pub mod len;
#[doc(inline)]
pub use len::{Len, LenExpr};

/// Marker trait for expressions that evaluate into boolean values
pub trait Numeric: CoreExpression {}

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
impl CoreExpression for i32 {}
impl Numeric for i32 {}
