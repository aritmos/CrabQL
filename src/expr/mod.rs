/// Expressions that can evaluate to any standard (non-custom) type
pub mod any;
/// Expressions that evaluate into boolean values
pub mod bool;
/// Expressions that evaluate into custom types
pub mod misc;
/// Expressions that evaluate into numeric values
pub mod num;
/// Expressions that evaluate into textual values
pub mod text;

/// Prelude for expression definitions
mod prelude;

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
    /// Returns the conditions that need to be verified by a [`Checker`]
    /// in order for the expression to be correct
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition>>;

    // RFC: replace dialect with `ops` that wraps dialect as well as if this is the outer
    // expression so the aliasing shows
    /// Returns the `String` representation of the expression in the given dialect
    fn display(&self, dialect: Dialect) -> String;

    // Modify the display name of the expression.
    //
    // This modification is only visible on outer expressions,
    // i.e. it means nothing on inner expressions
    // fn alias(&mut self, id: String);
}
