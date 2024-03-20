//! Expressions and functions

pub mod any;
pub mod bool;
pub mod misc;
pub mod num;
pub mod text;

/// Prelude for expression definitions
mod prelude;

use super::checker::Condition;

/// The possible evaluation types of an expression.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ExprType {
    /// Expressions that can return any other return type in this enum.
    /// It exists as a "nullop" within coercion environments.
    Any,
    /// Expressions that return boolean values
    Bool,
    /// Expressions that return numeric values
    Num,
    /// Expressions that return textual values
    Text,
}

/// Supported dialects, used for expression to string conversion
// RFC: these might also need to be used to validate the actual expressions
// themselves as not all dialects support all operations
#[derive(Copy, Clone)]
pub enum Dialect {
    Postgres,
}

/// Common functionality for expressions.
pub trait Expression {
    /// Returns the conditions that need to be verified by a [`Checker`][crate::checker::Checker]
    /// in order for the expression to be correct
    fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_>;

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

// These two traits are mutually exclusive!
// If Rust allowed it, these two traits could be collapsed into one and we could
// use the anti-trait pattern: `!BasicExpression` to refer to custom expressions

/// Expressions that evaluate into DB primitive types
pub trait CoreExpression: Expression {
    /// The evaluation type of the expression.
    ///
    /// Note: Requires `&self` so the trait remains object safe.
    fn eval_type(&self) -> ExprType;
}
/// Expressions that do not evaluate into DB primitive types
pub trait MiscExpression: Expression {}

// Boxed expressions are expressions
// impl Expression for Box<dyn Expression> {
//     fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
//         self.as_ref().conditions(coerce)
//     }
//
//     fn display(&self, dialect: Dialect) -> String {
//         self.as_ref().display(dialect)
//     }
// }
