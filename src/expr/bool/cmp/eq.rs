use crate::expr::prelude::*;

pub struct EqExpr {
    // `dyn Expression` as we can have equality for `(num, num)`, `(text, text)` etc
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
    // keep track of the inner expression types
    kind: ExprType,
}

impl Expression for EqExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Bool));

        let conds = [&self.lhs, &self.rhs]
            .into_iter()
            .flat_map(|e| e.conditions(self.kind));

        Box::new(conds)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} = {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl Boolean for EqExpr {}

pub trait NumEq<R> {
    fn eq(self, rhs: R) -> EqExpr;
}

impl<L, R> NumEq<R> for L
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    fn eq(self, rhs: R) -> EqExpr {
        EqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Num,
        }
    }
}

pub trait TextEq<R> {
    fn eq(self, rhs: R) -> EqExpr;
}

impl<L, R> TextEq<R> for L
where
    L: Textual + 'static,
    R: Textual + 'static,
{
    fn eq(self, rhs: R) -> EqExpr {
        EqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Text,
        }
    }
}

pub trait BoolEq<R> {
    fn eq(self, rhs: R) -> EqExpr;
}

impl<L, R> BoolEq<R> for L
where
    L: Boolean + 'static,
    R: Boolean + 'static,
{
    fn eq(self, rhs: R) -> EqExpr {
        EqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Bool,
        }
    }
}

pub trait AnyEq<R> {
    fn eq(self, rhs: R) -> EqExpr;
}

impl<L, R> AnyEq<R> for L
where
    L: Anything + 'static,
    R: Anything + 'static,
{
    fn eq(self, rhs: R) -> EqExpr {
        EqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Any,
        }
    }
}
