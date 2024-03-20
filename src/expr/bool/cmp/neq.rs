use crate::expr::prelude::*;

#[derive(IntoMultiCore)]
pub struct NEqExpr {
    // `dyn Expression` as we can have equality for `(num, num)`, `(text, text)` etc
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
    // keep track of the inner expression types
    kind: ExprType,
}

impl Expression for NEqExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Bool));

        let conds = [&self.lhs, &self.rhs]
            .into_iter()
            .flat_map(|e| e.conditions(self.kind));

        Box::new(conds)
    }

    fn display(&self, dialect: Dialect) -> String {
        format!(
            "{} != {}",
            self.lhs.display(dialect),
            self.rhs.display(dialect)
        )
    }
}
impl CoreExpression for NEqExpr {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }
}
impl Boolean for NEqExpr {}
super::super::logic::impl_bool_logic!(NEqExpr);

pub trait NumNEq<R> {
    fn neq(self, rhs: R) -> NEqExpr;
}

impl<L, R> NumNEq<R> for L
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    fn neq(self, rhs: R) -> NEqExpr {
        NEqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Num,
        }
    }
}

pub trait TextNEq<R> {
    fn neq(self, rhs: R) -> NEqExpr;
}

impl<L, R> TextNEq<R> for L
where
    L: Textual + 'static,
    R: Textual + 'static,
{
    fn neq(self, rhs: R) -> NEqExpr {
        NEqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Text,
        }
    }
}

pub trait BoolNEq<R> {
    fn neq(self, rhs: R) -> NEqExpr;
}

impl<L, R> BoolNEq<R> for L
where
    L: Boolean + 'static,
    R: Boolean + 'static,
{
    fn neq(self, rhs: R) -> NEqExpr {
        NEqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Bool,
        }
    }
}

pub trait AnyNEq<R> {
    fn neq(self, rhs: R) -> NEqExpr;
}

impl<L, R> AnyNEq<R> for L
where
    L: Anything + 'static,
    R: Anything + 'static,
{
    fn neq(self, rhs: R) -> NEqExpr {
        NEqExpr {
            lhs: Box::new(self),
            rhs: Box::new(rhs),
            kind: ExprType::Any,
        }
    }
}
