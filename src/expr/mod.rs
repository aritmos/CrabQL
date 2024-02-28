/// Expressions that can evaluate to any standard (non-custom) type
mod any;
/// Expressions that evaluate into boolean values
mod bool;
/// Expressions that evaluate into custom types
mod misc;
/// Expressions that evaluate into numeric values
mod num;
/// Expressions that evaluate into textual values
mod text;

use super::checker::Condition;

/// The possible evaluation types of an expression.
///
/// `Any` is an expression that can return any other return type in this enum.
/// It exists as a "nullop" within coercion environments.
pub enum ExprType {
    // Expressions that can return any type, such as columns and `CASE`s.
    Any,
    // Expressions that return boolean values
    Bool,
    // Expressions that return numeric values
    Num,
    // Expressions that return textual values
    Text,
}

/// Supported dialects, used for expression to string conversion
// RFC: these might also need to be used to validate the actual expressions
// themselves as not all dialects support all operations
pub enum Dialect {
    Postgres,
}

/// Common functionality for expressions.
pub trait Expression {
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition>>;

    fn display(&self, dialect: Dialect) -> String;
}
