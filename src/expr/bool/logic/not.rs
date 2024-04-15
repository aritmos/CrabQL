use crate::expr::prelude::*;

pub struct Not {
    inner: Box<dyn Expression>, // Boolean
}

impl Not {
    pub fn new(inner: Box<dyn Expression>) -> Self {
        Self { inner }
    }
}

impl Client for Not {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        debug_assert!(matches!(ctx, ExprType::Any | ExprType::Bool));
        vec![(self.inner.as_ref(), ExprType::Bool)]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for Not {}
impl Expression for Not {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    // TODO: Account for "IS NOT NULL" for `IS`-type of expressions
    fn display(&self, dialect: Dialect) -> String {
        format!("NOT {}", self.inner.display(dialect))
    }
}
impl Common for Not {}
impl Boolean for Not {}
