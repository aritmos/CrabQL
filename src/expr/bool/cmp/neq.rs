use crate::expr::prelude::*;

pub struct Neq {
    // `dyn Expression` as we can have equality for `(num, num)`, `(text, text)` etc
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
    // keep track of the inner expression types
    kind: ExprType,
}

impl Neq {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>, kind: ExprType) -> Self {
        Self { lhs, rhs, kind }
    }
}

impl Client for Neq {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        vec![
            (self.lhs.as_ref(), self.kind),
            (self.rhs.as_ref(), self.kind),
        ]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for Neq {}
impl Expression for Neq {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} != {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl Common for Neq {}
impl Boolean for Neq {}
