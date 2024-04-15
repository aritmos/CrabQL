pub mod any;
pub mod bool;
pub mod common;
pub mod num;
mod prelude;
pub mod text;
pub mod unique;

use std::ops::Deref;

use super::checker::{Checkable, Message};

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
    /// Unique expressions
    Unique,
}

/// Supported dialects, used for expression to string conversion
// RFC: these might also need to be used to validate the actual expressions
// themselves as not all dialects support all operations
#[derive(Copy, Clone)]
pub enum Dialect {
    Postgres,
}

/// Common additional functionality for expressions.
///
/// As the validation side is covered by `Checkable`,
/// the only required functionality is the transformation into a string.
pub trait Expression: Checkable {
    /// The evaluation type of the expression.
    fn eval_type(&self) -> ExprType;

    /// Returns the `String` representation of the expression in the given dialect
    fn display(&self, dialect: Dialect) -> String;
}

// Expressions are wrapped in types (separate ones depending on if they are Common or not) to
// simplify the implementation of std::ops operators in a generic form.
// With this approach the generics are covered meaning there is no need for macro implementations
// of any kind.
//
// Both wrapping types could be consolidated into a single `Expr<T>` type if specialization (or the
// required subset) along with negative impls and bounds were completed features. With that one could do
// disjoint implementations as follows:
// ```rust
// impl<T: Common, U> std::ops::SomeOp<U> for Expr<T> {...}
// impl<T: !Common, U> std::ops::SomeOp<U> for Expr<T> {...}
// ```

/// Wrapper for `Common` expressions.
pub struct CommonExpr<T>(T);

impl<T> std::ops::Deref for CommonExpr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Life<'a>(std::marker::PhantomData<&'a ()>);

/// Wrapper for *unique* (non-`Common`) expressions.
pub struct UniqueExpr<T>(T);

impl<T> std::ops::Deref for UniqueExpr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
