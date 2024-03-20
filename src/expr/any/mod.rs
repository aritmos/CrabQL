//! Expressions that can evaluate to any standard (non-custom) type

use crate::expr::prelude::*;

/// Marker trait for expressions that can evaluate into any standard type
///
/// e.g. columns and `CASE`s
pub trait Anything: CoreExpression + Textual + Boolean + Numeric {}

pub mod col;
#[doc(inline)]
pub use col::{col, Column};

pub mod case;
