use crate::expr::prelude::*;

pub struct Or {
    lhs: Box<dyn Expression>, // Boolean
    rhs: Box<dyn Expression>, // Boolean
}

impl Or {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Self { lhs, rhs }
    }
}

impl Client for Or {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        debug_assert!(matches!(ctx, ExprType::Any | ExprType::Bool));
        vec![
            (self.lhs.as_ref(), ExprType::Bool),
            (self.rhs.as_ref(), ExprType::Bool),
        ]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for Or {}
impl Expression for Or {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} OR {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl Common for Or {}
impl Boolean for Or {}
