use crate::expr::prelude::*;

#[derive(IntoMultiCore)]
pub struct AndExpr {
    lhs: Box<dyn Boolean>,
    rhs: Box<dyn Boolean>,
}

impl AndExpr {
    pub fn new(lhs: Box<dyn Boolean>, rhs: Box<dyn Boolean>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expression for AndExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Bool));

        let conds = [&self.lhs, &self.rhs]
            .into_iter()
            .flat_map(|e| e.conditions(ExprType::Bool));
        Box::new(conds)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} AND {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl CoreExpression for AndExpr {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }
}
impl Boolean for AndExpr {}
super::impl_bool_logic!(AndExpr);

pub trait And<R> {
    fn and(self, rhs: R) -> AndExpr;
}

impl<L, R> And<R> for L
where
    L: Boolean + 'static,
    R: Boolean + 'static,
{
    fn and(self, rhs: R) -> AndExpr {
        AndExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
        }
    }
}
