//! Expressions that evaluate into boolean values

use super::CoreExpression;

/// Marker trait for expressions that evaluate into boolean values
pub trait Boolean: CoreExpression {}

/// Comparison expressions
pub mod cmp;
#[doc(inline)]
pub use cmp::{AnyEq, BoolEq, EqExpr, NumNEq, TextNEq};
#[doc(inline)]
pub use cmp::{AnyNEq, BoolNEq, NEqExpr, NumEq, TextEq};
#[doc(inline)]
pub use cmp::{GEq, GEqExpr};
#[doc(inline)]
pub use cmp::{GTExpr, GT};
#[doc(inline)]
pub use cmp::{LEq, LEqExpr};
#[doc(inline)]
pub use cmp::{LTExpr, LT};

/// Logical operator expressions
#[macro_use]
pub mod logic;
#[doc(inline)]
pub use logic::{AndExpr, NotExpr, OrExpr};

pub mod between;
#[doc(inline)]
pub use between::{Between, BetweenExpr};
