use crate::expr::prelude::*;

pub struct OrExpr {
    lhs: Box<dyn Boolean>,
    rhs: Box<dyn Boolean>,
}

impl OrExpr {
    pub fn new(lhs: Box<dyn Boolean>, rhs: Box<dyn Boolean>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expression for OrExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Bool));

        let conds = [&self.lhs, &self.rhs]
            .into_iter()
            .flat_map(|e| e.conditions(ExprType::Bool));
        Box::new(conds)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} OR {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl Boolean for OrExpr {}
super::impl_bool_logic!(OrExpr);

pub trait Or<R> {
    fn or(self, rhs: R) -> OrExpr;
}

impl<L, R> Or<R> for L
where
    L: Boolean + 'static,
    R: Boolean + 'static,
{
    fn or(self, rhs: R) -> OrExpr {
        OrExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}
