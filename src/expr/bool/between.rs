use crate::expr::prelude::*;

pub struct Between {
    inner: Box<dyn Expression>, // Numeric
    lower: Box<dyn Expression>, // Numeric
    upper: Box<dyn Expression>, // Numeric
}

impl Between {
    pub fn new(
        inner: Box<dyn Expression>,
        lower: Box<dyn Expression>,
        upper: Box<dyn Expression>,
    ) -> Self {
        Self {
            inner,
            lower,
            upper,
        }
    }
}

impl Client for Between {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        vec![
            (self.inner.as_ref(), ExprType::Num),
            (self.lower.as_ref(), ExprType::Num),
            (self.upper.as_ref(), ExprType::Num),
        ]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        todo!()
    }
}
impl Checkable for Between {}
impl Expression for Between {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} BETWEEN {} AND {}",
            self.inner.display(dialect),
            self.lower.display(dialect),
            self.upper.display(dialect)
        )
    }
}
impl Common for Between {}
impl Boolean for Between {}
