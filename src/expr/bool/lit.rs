use crate::expr::prelude::*;

impl Expression for bool {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        Box::new(std::iter::empty())
    }

    fn display(&self, dialect: Dialect) -> String {
        self.to_string()
    }
}
impl CoreExpression for bool {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }
}
impl Boolean for bool {}
