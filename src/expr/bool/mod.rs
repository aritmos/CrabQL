use super::Expression;

/// Marker trait for expressions that evaluate into boolean values
pub trait Boolean: Expression {}

/// Comparison expressions
pub mod cmp;
pub use cmp::*;

/// Logical operator expressions
pub mod logic;
pub use logic::*;

pub mod between;
pub use between::*;
