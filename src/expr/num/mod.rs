//! Expressions that evaluate into numeric values

use crate::expr::prelude::*;

pub mod arith;
#[doc(inline)]
pub use arith::{Add, Div, Mul, Rem, Sub};

pub mod len;
#[doc(inline)]
pub use len::Len;

pub mod lit;
#[doc(inline)]
pub use lit::*;

use super::bool;

/// Marker trait for expressions that evaluate into boolean values
pub trait Numeric: Common {
    fn add<R>(self, rhs: R) -> CommonExpr<Add>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(Add::new(Box::new(self), Box::new(rhs)))
    }

    fn sub<R>(self, rhs: R) -> CommonExpr<Sub>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(Sub::new(Box::new(self), Box::new(rhs)))
    }

    fn mul<R>(self, rhs: R) -> CommonExpr<Mul>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(Mul::new(Box::new(self), Box::new(rhs)))
    }

    fn div<R>(self, rhs: R) -> CommonExpr<Div>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(Div::new(Box::new(self), Box::new(rhs)))
    }

    fn rem<R>(self, rhs: R) -> CommonExpr<Rem>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(Rem::new(Box::new(self), Box::new(rhs)))
    }

    fn eq<R>(self, rhs: R) -> CommonExpr<bool::Eq>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(bool::Eq::new(Box::new(self), Box::new(rhs), ExprType::Num))
    }

    fn neq<R>(self, rhs: R) -> CommonExpr<bool::Neq>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(bool::Neq::new(Box::new(self), Box::new(rhs), ExprType::Num))
    }

    fn gt<R>(self, rhs: R) -> CommonExpr<bool::Gt>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(bool::Gt::new(Box::new(self), Box::new(rhs)))
    }

    fn ge<R>(self, rhs: R) -> CommonExpr<bool::Geq>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(bool::Geq::new(Box::new(self), Box::new(rhs)))
    }

    fn lt<R>(self, rhs: R) -> CommonExpr<bool::Lt>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(bool::Lt::new(Box::new(self), Box::new(rhs)))
    }

    fn leq<R>(self, rhs: R) -> CommonExpr<bool::Leq>
    where
        Self: Sized + 'static,
        R: Numeric + 'static,
    {
        CommonExpr(bool::Leq::new(Box::new(self), Box::new(rhs)))
    }

    fn between<L, U>(self, lower: L, upper: U) -> CommonExpr<bool::Between>
    where
        Self: Sized + 'static,
        L: Numeric + 'static,
        U: Numeric + 'static,
    {
        CommonExpr(bool::Between::new(
            Box::new(self),
            Box::new(lower),
            Box::new(upper),
        ))
    }

    fn within<Idx>(self, range: std::ops::RangeInclusive<Idx>) -> CommonExpr<bool::Between>
    where
        Self: Sized + 'static,
        Idx: Numeric + 'static,
    {
        let (lower, upper) = range.into_inner();
        self.between(lower, upper)
    }
}
impl<T: Numeric> Numeric for CommonExpr<T> {}
