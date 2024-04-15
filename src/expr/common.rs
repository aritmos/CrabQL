use crate::expr::prelude::*;

impl<T: Client> Client for CommonExpr<T> {
    type Ctx = T::Ctx;
    type Msg = T::Msg;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        self.0.children(ctx)
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        self.0.messages(ctx)
    }

    fn send_all(&self, ctx: Self::Ctx, server: &mut dyn Server<Msg = Self::Msg>) {
        self.0.send_all(ctx, server)
    }
}
impl<T: Checkable> Checkable for CommonExpr<T> {}
impl<T: Expression> Expression for CommonExpr<T> {
    fn eval_type(&self) -> ExprType {
        self.0.eval_type()
    }

    fn display(&self, dialect: Dialect) -> String {
        self.0.display(dialect)
    }
}

// Common::eval_type is used by `CommonExpr<Case>` to assess if it can directly downcast from `Anything` into
// a more specific type.
/// Expressions that evaluate into DB primitive types: boolean, numeric, textual, temporal, etc.
pub trait Common: Expression {
    fn alias(self, s: impl Into<String>) -> CommonExpr<Alias>
    where
        Self: Sized + 'static,
    {
        CommonExpr(Alias::new(Box::new(self), s.into()))
    }
}
impl<T: Common> Common for CommonExpr<T> {}

pub struct Alias {
    inner: Box<dyn Expression>,
    alias: String,
}

impl Alias {
    pub fn new(inner: Box<dyn Expression>, alias: impl Into<String>) -> Self {
        Self {
            inner,
            alias: alias.into(),
        }
    }
}

impl Client for Alias {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        vec![(self.inner.as_ref(), ctx)]
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for Alias {}
impl Expression for Alias {
    fn eval_type(&self) -> ExprType {
        self.inner.eval_type()
    }

    fn display(&self, dialect: Dialect) -> String {
        format!("{} AS {}", self.inner.display(dialect), self.alias)
    }
}
impl Common for Alias {}
