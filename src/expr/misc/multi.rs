#![allow(clippy::vec_init_then_push)]
use crate::expr::prelude::*;

/// An expression encapsulating an ordered collection of `Expression`s.
///
/// This is the only type that implements `MiscExpression` but not `IntoMultiMisc`.
pub struct MultiExpr<T> {
    exprs: Vec<T>,
}

impl<T> From<Vec<T>> for MultiExpr<T> {
    fn from(value: Vec<T>) -> Self {
        MultiExpr { exprs: value }
    }
}

impl<T> MultiExpr<T> {
    pub fn new(expr: T) -> MultiExpr<T> {
        Self { exprs: vec![expr] }
    }

    pub fn push(&mut self, expr: T) {
        self.exprs.push(expr);
    }
}

impl<T: MiscExpression> Expression for MultiExpr<T> {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(coerce == ExprType::Any);

        let conds = self.exprs.iter().flat_map(|e| e.conditions(ExprType::Any));
        Box::new(conds)
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

impl Expression for MultiExpr<Box<dyn CoreExpression>> {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
        debug_assert!(coerce == ExprType::Any);

        let conds = self.exprs.iter().flat_map(|e| e.conditions(ExprType::Any));
        Box::new(conds)
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

/// A trait defining how types get pushed into a `MultiExpr`
/// All implementors of this trait are `Expression`s, namely
/// - `T: CoreExpression` <=> `T: IntoMulti<Box<dyn CoreExpression>>`
/// - `T: MiscExpression` <=> `T: IntoMulti<T>`
///
/// The trait is not defined as a supertrait of these expressions traits for two reasons
/// 1. The traits would no longer be object safe
/// 2. The traits become self-referential (cyclical) and therefore can't even compile
///
/// This trait is only meant to be derived via the respective proc macros.
/// Since proc macros don't currently support type parameters in traits
/// the following wrapper traits have been defined:
/// - [`IntoMultiCore`] : `IntoMulti<Box<dyn CoreExpression>>`
/// - [`IntoMultiMisc`] : `IntoMulti<Self>`
///
/// ```rust
/// // allow for this (core) expression be added to multi-expressions of core-expressions
/// #[derive(IntoMultiCore)]
/// pub struct SomeExpr {...}
/// impl Expression for SomeExpr {...}
/// impl CoreExpression for SomeExpr {...}
/// ...
/// ```
pub trait IntoMulti<T>
where
    Self: Sized,
    (): std::ops::Shl<Self, Output = MultiExpr<T>>,
    MultiExpr<T>: std::ops::Shl<Self, Output = MultiExpr<T>>,
{
}

/// A marker trait for `IntoMulti<Box<dyn CoreExpression>>`
/// (as derive macros don't support generics)
pub trait IntoMultiCore: IntoMulti<Box<dyn CoreExpression>>
where
    Self: Sized,
    (): std::ops::Shl<Self, Output = MultiExpr<Box<dyn CoreExpression>>>,
    MultiExpr<Box<dyn CoreExpression>>:
        std::ops::Shl<Self, Output = MultiExpr<Box<dyn CoreExpression>>>,
{
}

/// A marker trait for `IntoMulti<Self>`
/// (as derive macros don't support generics)
pub trait IntoMultiMisc: IntoMulti<Self>
where
    Self: Sized,
    (): std::ops::Shl<Self, Output = MultiExpr<Self>>,
    MultiExpr<Self>: std::ops::Shl<Self, Output = MultiExpr<Self>>,
{
}

/// Joins all of the inner expressions into a `MultiExpr`.
///
/// Wrapper for the alternative shift left notation.
#[macro_export]
macro_rules! multi {
    ($($e:expr),+) => {{
        let multi = ();
        $(let multi = multi << $e;)+
        multi
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expression;

    #[test]
    fn display() {
        use crate::expr::{any::col::col, bool::GT, num::len::Len};
        let _a = () << col("hello") << col("world").len().gt(3);
        let _b = multi![col("hello"), col("world").len().gt(3)];
        println!("SELECT {} FROM my_table", _a.display(Dialect::Postgres));
    }
}
