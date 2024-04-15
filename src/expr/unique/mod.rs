//! Expressions that evaluate into custom types

use crate::expr::prelude::*;

#[macro_use]
pub mod multi;
#[doc(inline)]
pub use multi::Multi;

// this is done so we can have a bound on the Shl overloads,
// stopping the ability of having `UniqueExpr<Multi<UniqueExpr<Multi<_>>>>`
/// Unique expressions that can be put inside of a `UniqueExpr<Multi<T>>`.
pub trait UniqueMultiInner: Expression {}
