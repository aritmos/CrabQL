use crate::expr::prelude::*;

pub struct Len {
    inner: Box<dyn Expression>, // Textual
}

impl Len {
    pub fn new(inner: Box<dyn Expression>) -> Self {
        Self { inner }
    }
}

impl Client for Len {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        debug_assert!(matches!(ctx, ExprType::Any | ExprType::Num));
        vec![(self.inner.as_ref(), ExprType::Text)]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for Len {}
impl Expression for Len {
    fn eval_type(&self) -> ExprType {
        ExprType::Num
    }

    fn display(&self, dialect: Dialect) -> String {
        match dialect {
            Dialect::Postgres => format!("LENGTH({})", self.inner.display(dialect)),
        }
    }
}
impl Common for Len {}
impl Numeric for Len {}
