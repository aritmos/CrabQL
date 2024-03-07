use crate::expr::prelude::*;
// use crabql_derive::IntoMultiCore;

#[derive(IntoMultiCore)]
pub struct AliasExpr {
    inner: Box<dyn CoreExpression>,
    alias: String,
}

impl Expression for AliasExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        self.inner.conditions(coerce)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!("{} AS {}", self.inner.display(dialect), self.alias)
    }
}
impl CoreExpression for AliasExpr {}
