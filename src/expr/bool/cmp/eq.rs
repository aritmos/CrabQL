use crate::expr::prelude::*;

/// An equality operation.
/// This struct is common to all equality operations regardless of the CommonType.
/// Instead the CommonType is stored internally.
pub struct Eq {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
    kind: ExprType,
}

impl Eq {
    pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>, kind: ExprType) -> Self {
        Self { lhs, rhs, kind }
    }
}
impl Client for Eq {
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
        unreachable!();
    }

    fn send_all(&self, ctx: Self::Ctx, server: &mut dyn Server<Msg = Self::Msg>) {
        if self.kind == ExprType::Any {
            server.accept(Signal::StartLink.into());
            for (child, ctx) in self.children(ctx) {
                child.send_all(ctx, server);
            }
            server.accept(Signal::EndLink.into());
        } else {
            for (child, ctx) in self.children(ctx) {
                child.send_all(ctx, server);
            }
        }
    }
}
impl Checkable for Eq {}
impl Expression for Eq {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} = {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl Common for Eq {}
impl Boolean for Eq {}
