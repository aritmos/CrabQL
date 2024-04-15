use crate::expr::prelude::*;

pub struct And {
    lhs: Box<dyn Expression>, // Boolean
    rhs: Box<dyn Expression>, // Boolean
}

impl And {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Self { lhs, rhs }
    }
}

impl Client for And {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        vec![
            (self.lhs.as_ref(), ExprType::Bool),
            (self.rhs.as_ref(), ExprType::Bool),
        ]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        debug_assert!(matches!(ctx, ExprType::Any | ExprType::Bool));
        Vec::new()
    }
}
impl Checkable for And {}
impl Expression for And {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} AND {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl Common for And {}
impl Boolean for And {}
