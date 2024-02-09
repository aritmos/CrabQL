#![allow(unused)]
use std::ops::{BitAnd, BitOr};

use super::{
    column::Column,
    funcs::{Func, Functions},
    value,
    value::Value,
};

/// A possibly incorrect expression.
/// The type's API is constructed such that any incorrect expression building is kept internally
/// until manually requested. This allows for a cleaner interface when combining expressions.
pub struct ExprResult {
    /// A correct inner expression.
    /// # Safety:
    /// - If `self.error.is_none()` this expression is guaranteed to be the result of all of the
    /// operations that have been applied to create this type.
    ///
    /// - If `self.error.is_some()` this expression is not guaranteed to be the result defined above.
    ///
    /// Basically once the `ExprResult` encounters an error, it can stop updating the `self.expr`
    /// field.
    pub(super) expr: Expr,
    // RFC: do we prefer keeping track of multiple errors?
    pub(super) error: Option<ExprErr>,
}

/// A generic expression
// NOTE: The internal representation has been created in such a way as for incorrect expressions
// to be allowed. This is the intended behaviour as we want only the checker to internally keep
// track of correctness instead of having to handle errors as the expression is created.
pub enum Expr {
    /// The base building block of expressions, variables or literals: `col`, `3`, ...
    Base(BaseExpr),
    /// An expression that signifies a boolean scenario: `col > 3`
    Bool(BoolExpr),
    /// An expression that is built from `BaseExpr`s and other `FuncExpr`s via functions:
    ///  `LEN(col) * 2`
    Func(FuncExpr),
}

// TODO: Change this into an enum.
pub struct ExprErr(pub(super) String);

pub struct BoolExpr {
    pub(super) lhs: Box<Expr>,
    pub(super) op: BoolOp,
    pub(super) rhs: Box<Expr>,
}

/// Boolean operations that can be applied to expressions
pub enum BoolOp {
    // These are used to create the BoolExpr
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
    // These are used to combine BoolExpr
    And,
    Or,
}

pub struct FuncExpr {
    pub(super) func: Func,
    pub(super) args: Vec<Expr>,
}

pub enum BaseExpr {
    Col(String),
    Lit(Value),
}

impl<'c> From<Column<'c>> for ExprResult {
    fn from(column: Column<'c>) -> Self {
        ExprResult {
            expr: Expr::Base(BaseExpr::Col(column.name.to_string())),
            error: None,
        }
    }
}

impl From<usize> for ExprResult {
    fn from(value: usize) -> Self {
        ExprResult {
            expr: Expr::Base(BaseExpr::Lit(Value::Int(value as isize))),
            error: None,
        }
    }
}

impl BitAnd for ExprResult {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for ExprResult {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl Functions for ExprResult {}
