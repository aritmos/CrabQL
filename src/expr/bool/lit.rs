use crate::expr::prelude::*;

// TODO: add ctx safety checks
impl Client for bool {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        Vec::new()
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for bool {}
impl Expression for bool {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    fn display(&self, dialect: Dialect) -> String {
        self.to_string()
    }
}
impl Common for bool {}
impl Boolean for bool {}
