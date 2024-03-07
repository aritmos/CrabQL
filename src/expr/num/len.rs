use crate::expr::prelude::*;

pub struct LenExpr {
    inner: Box<dyn Textual>,
}

impl Expression for LenExpr {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(matches!(coerce, ExprType::Any | ExprType::Num));
        self.inner.conditions(ExprType::Text)
    }

    fn display(&self, dialect: Dialect) -> String {
        match dialect {
            Dialect::Postgres => format!("LENGTH({})", self.inner.display(dialect)),
        }
    }
}
impl CoreExpression for LenExpr {}
impl Numeric for LenExpr {}
super::arith::impl_arith_ops!(LenExpr);

#[allow(clippy::len_without_is_empty)]
pub trait Len {
    fn len(self) -> LenExpr;
}

impl<T: Textual + 'static> Len for T {
    fn len(self) -> LenExpr {
        LenExpr {
            inner: Box::new(self),
        }
    }
}
