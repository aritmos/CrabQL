//! Expressions that evaluate into textual values
use crate::expr::prelude::*;

use super::{bool, num};

/// Marker trait for expressions that evaluate into boolean values
pub trait Textual: Common {
    #![allow(clippy::len_without_is_empty)]
    fn len(self) -> CommonExpr<num::Len>
    where
        Self: Sized + 'static,
    {
        CommonExpr(num::Len::new(Box::new(self)))
    }

    fn eq<R>(self, rhs: R) -> CommonExpr<bool::Eq>
    where
        Self: Sized + 'static,
        R: Textual + 'static,
    {
        CommonExpr(bool::Eq::new(Box::new(self), Box::new(rhs), ExprType::Text))
    }

    fn neq<R>(self, rhs: R) -> CommonExpr<bool::Neq>
    where
        Self: Sized + 'static,
        R: Textual + 'static,
    {
        CommonExpr(bool::Neq::new(
            Box::new(self),
            Box::new(rhs),
            ExprType::Text,
        ))
    }
}
impl<T: Textual> Textual for CommonExpr<T> {}
