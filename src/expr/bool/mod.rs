use super::Expression;

/// Marker trait for expressions that evaluate into boolean values
pub trait Boolean: Expression {}

/// Comparison expressions
pub mod cmp;
pub use cmp::*;
