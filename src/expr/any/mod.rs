use super::Expression;

/// Marker trait for expressions that can evaluate into any standard type
///
/// e.g. columns and `CASE`s
pub trait Anything: Expression {}

pub mod col;
