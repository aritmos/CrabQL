use super::UniqueMultiInner;
use crate::expr::prelude::*;

/// Multi-expressions made up of `Common` expressions.
pub struct MultiCommon {
    exprs: Vec<Box<dyn Common>>,
}

impl From<Vec<Box<dyn Common>>> for MultiCommon {
    fn from(value: Vec<Box<dyn Common>>) -> Self {
        MultiCommon { exprs: value }
    }
}

impl MultiCommon {
    pub fn new(expr: Box<dyn Common>) -> Self {
        Self { exprs: vec![expr] }
    }

    pub fn push(&mut self, expr: Box<dyn Common>) {
        self.exprs.push(expr);
    }
}

impl Client for MultiCommon {
    type Ctx = ExprType;
    type Msg = Message;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        self.exprs
            .iter()
            .map(|common_expr| (common_expr.as_ref() as _, ctx))
            .collect()
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl Checkable for MultiCommon {}
impl Expression for MultiCommon {
    fn eval_type(&self) -> ExprType {
        /// RFC: Should we create a new variant for this case?
        ExprType::Unique
    }

    fn display(&self, dialect: Dialect) -> String {
        debug_assert!(!self.exprs.is_empty());

        let mut iter = self.exprs.iter();

        let mut out = match iter.next() {
            Some(expr) => expr.display(dialect),
            None => unreachable!("MultiExpr with no expressions can't exist"),
        };

        for expr in iter {
            out += ", ";
            out += &expr.display(dialect);
        }

        out
    }
}

/// Multi-expressions for a list of `UniqueExpr`s of the same kind.
pub struct Multi<T> {
    exprs: Vec<T>,
}

impl<T> From<Vec<T>> for Multi<T> {
    fn from(value: Vec<T>) -> Self {
        Multi { exprs: value }
    }
}

impl<T> Multi<T> {
    pub fn new(expr: T) -> Multi<T> {
        Self { exprs: vec![expr] }
    }

    pub fn push(&mut self, expr: T) {
        self.exprs.push(expr);
    }
}

impl<T> Client for Multi<T>
where
    T: Client,
{
    type Ctx = T::Ctx;
    type Msg = T::Msg;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
        self.exprs
            .iter()
            .map(|unique_expr| (unique_expr as _, ctx))
            .collect()
    }

    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
        Vec::new()
    }
}
impl<T: Checkable> Checkable for Multi<T> {}
impl<T: Expression> Expression for Multi<T> {
    fn eval_type(&self) -> ExprType {
        ExprType::Unique
    }

    fn display(&self, dialect: Dialect) -> String {
        debug_assert!(!self.exprs.is_empty());

        let mut iter = self.exprs.iter();

        let mut out = match iter.next() {
            Some(expr) => expr.display(dialect),
            None => unreachable!("MultiExpr with no expressions can't exist"),
        };

        for expr in iter {
            out += ", ";
            out += &expr.display(dialect);
        }

        out
    }
}

impl<T: Common + 'static> std::ops::Shl<CommonExpr<T>> for () {
    type Output = UniqueExpr<MultiCommon>;

    fn shl(self, rhs: CommonExpr<T>) -> Self::Output {
        UniqueExpr(MultiCommon::new(Box::new(rhs)))
    }
}

impl<T: Common + 'static> std::ops::Shl<CommonExpr<T>> for UniqueExpr<MultiCommon> {
    type Output = UniqueExpr<MultiCommon>;

    fn shl(self, rhs: CommonExpr<T>) -> Self::Output {
        let mut multi = self;
        multi.0.push(Box::new(rhs));
        multi
    }
}

impl<T: UniqueMultiInner> std::ops::Shl<UniqueExpr<T>> for () {
    type Output = UniqueExpr<Multi<T>>;

    fn shl(self, rhs: UniqueExpr<T>) -> Self::Output {
        UniqueExpr(Multi::new(rhs.0))
    }
}

impl<T: UniqueMultiInner> std::ops::Shl<UniqueExpr<T>> for UniqueExpr<Multi<T>> {
    type Output = UniqueExpr<Multi<T>>;

    fn shl(self, rhs: UniqueExpr<T>) -> Self::Output {
        let mut multi = self;
        multi.0.push(rhs.0);
        multi
    }
}

/// Wrapper for the alternative `() <<` notation of creating multi-expressions.
/// Returns a `UniqueExpr<MultiCommon>` or `UniqueExpr<Multi<T>>` based on what is passed in.
#[macro_export]
macro_rules! multi {
    ($($e:expr),+) => {{
        // let multi = ();
        // $(let multi = multi << $e;)+
        // multi
        () $(<< $e)+
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expression;

    #[test]
    fn display() {
        use crate::expr::{any::col, bool::Gt, num::Numeric};
        let _a = () << col("hello") << col("world").len().gt(3);
        let _b = multi![col("hello"), col("world").len().gt(3)];
        println!("SELECT {} FROM my_table", _b.display(Dialect::Postgres));
    }
}
