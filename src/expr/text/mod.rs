//! Expressions that evaluate into textual values

use super::Expression;

/// Marker trait for expressions that evaluate into boolean values
pub trait Textual: Expression {}
