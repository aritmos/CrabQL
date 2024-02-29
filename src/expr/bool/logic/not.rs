use crate::expr::prelude::*;

pub struct NotExpr {
    inner: Box<dyn Boolean>,
}

// impl NotExpr {
//     pub fn new(lhs: Box<dyn Boolean>, rhs: Box<dyn Boolean>) -> Self {
//         Self { lhs, rhs }
//     }
// }

impl Expression for NotExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Bool));

        let conds = [&self.inner]
            .into_iter()
            .flat_map(|e| e.conditions(ExprType::Bool));
        Box::new(conds)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!("NOT {}", self.inner.display(dialect))
    }
}
impl Boolean for NotExpr {}
super::impl_bool_logic!(NotExpr);

pub trait Not {
    fn not(self) -> NotExpr;
}

impl<L> Not for L
where
    L: Boolean + 'static,
{
    fn not(self) -> NotExpr {
        NotExpr {
            inner: Box::new(self),
        }
    }
}
