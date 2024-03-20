use crate::expr::prelude::*;

#[derive(IntoMultiCore)]
pub struct BetweenExpr {
    inner: Box<dyn Numeric>,
    lower: Box<dyn Numeric>,
    upper: Box<dyn Numeric>,
}

impl Expression for BetweenExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Num));

        let conds = [&self.inner, &self.lower, &self.upper]
            .into_iter()
            .flat_map(|e| e.conditions(ExprType::Num));

        Box::new(conds)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} BETWEEN {} AND {}",
            self.inner.display(dialect),
            self.lower.display(dialect),
            self.upper.display(dialect)
        )
    }
}
impl CoreExpression for BetweenExpr {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }
}
impl Boolean for BetweenExpr {}
super::logic::impl_bool_logic!(BetweenExpr);

pub trait Between<L, U> {
    fn between(self, lower: L, upper: U) -> BetweenExpr;
}

impl<T, L, U> Between<L, U> for T
where
    T: Numeric + 'static,
    L: Numeric + 'static,
    U: Numeric + 'static,
{
    fn between(self, lower: L, upper: U) -> BetweenExpr {
        BetweenExpr {
            inner: Box::new(self),
            lower: Box::new(lower),
            upper: Box::new(upper),
        }
    }
}

/// A trait like [`Between`] but allowing for ranges to be used
pub trait Within: Between<i32, i32> {
    fn within(self, range: std::ops::RangeInclusive<i32>) -> BetweenExpr
    where
        Self: Sized,
    {
        let (lower, upper) = range.into_inner();
        self.between(lower, upper)
    }
}
