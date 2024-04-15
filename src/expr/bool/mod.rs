//! Expressions that evaluate into boolean values

use crate::expr::prelude::*;

/// Common expressions that evaluate into boolean values.
pub trait Boolean: Common {
    fn eq<R>(self, rhs: R) -> CommonExpr<Eq>
    where
        Self: Sized + 'static,
        R: Boolean + 'static,
    {
        CommonExpr(Eq::new(Box::new(self), Box::new(rhs), ExprType::Bool))
    }

    fn neq<R>(self, rhs: R) -> CommonExpr<Neq>
    where
        Self: Sized + 'static,
        R: Boolean + 'static,
    {
        CommonExpr(Neq::new(Box::new(self), Box::new(rhs), ExprType::Bool))
    }
}
impl<T: Boolean> Boolean for CommonExpr<T> {}

/// Comparison expressions
pub mod cmp;
#[doc(inline)]
pub use cmp::{Eq, Geq, Gt, Leq, Lt, Neq};

/// Logical operator expressions
#[macro_use]
pub mod logic;
#[doc(inline)]
pub use logic::{And, Not, Or};

pub mod between;
#[doc(inline)]
pub use between::Between;

pub mod lit;
