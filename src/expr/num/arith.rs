//! Arithmetic operators as expressions

use crate::expr::{any::col::Column, prelude::*};

macro_rules! impl_arith_expr {
    ($struct:ident, $trait:ident, $method:ident, $display:expr) => {
        pub struct $struct {
            lhs: Box<dyn Numeric>,
            rhs: Box<dyn Numeric>,
        }

        impl $struct {
            pub fn new(lhs: Box<dyn Numeric>, rhs: Box<dyn Numeric>) -> Self {
                Self { lhs, rhs }
            }
        }

        impl Expression for $struct {
            fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
                debug_assert!(matches!(coerce, ExprType::Any | ExprType::Num));

                let conds = [&self.lhs, &self.rhs]
                    .into_iter()
                    .flat_map(|e| e.conditions(ExprType::Num));
                Box::new(conds)
            }

            fn display(&self, dialect: Dialect) -> String {
                format!(
                    $display,
                    self.lhs.display(dialect),
                    self.rhs.display(dialect)
                )
            }
        }
        impl CoreExpression for $struct {
            fn eval_type(&self) -> ExprType {
                ExprType::Num
            }
        }
        impl Numeric for $struct {}

        pub trait $trait<R> {
            fn $method(self, rhs: R) -> $struct;
        }

        impl<L, R> $trait<R> for L
        where
            L: Numeric + 'static,
            R: Numeric + 'static,
        {
            fn $method(self, rhs: R) -> $struct {
                $struct::new(Box::new(self), Box::new(rhs))
            }
        }
    };
}

impl_arith_expr!(AddExpr, Add, add, "({} + {})");
impl_arith_expr!(SubExpr, Sub, sub, "({} - {})");
impl_arith_expr!(MulExpr, Mul, mul, "({} * {})");
impl_arith_expr!(DivExpr, Div, div, "({} / {})");
impl_arith_expr!(RemExpr, Rem, rem, "({} % {})");

macro_rules! impl_arith_ops {
    ($struct:ident) => {
        impl<R: Numeric + 'static> std::ops::Add<R> for $struct {
            type Output = crate::expr::num::arith::AddExpr;

            fn add(self, rhs: R) -> Self::Output {
                crate::expr::num::arith::AddExpr::new(Box::new(self), Box::new(rhs))
            }
        }

        impl<R: Numeric + 'static> std::ops::Sub<R> for $struct {
            type Output = crate::expr::num::arith::SubExpr;

            fn sub(self, rhs: R) -> Self::Output {
                crate::expr::num::arith::SubExpr::new(Box::new(self), Box::new(rhs))
            }
        }

        impl<R: Numeric + 'static> std::ops::Mul<R> for $struct {
            type Output = crate::expr::num::arith::MulExpr;

            fn mul(self, rhs: R) -> Self::Output {
                crate::expr::num::arith::MulExpr::new(Box::new(self), Box::new(rhs))
            }
        }

        impl<R: Numeric + 'static> std::ops::Div<R> for $struct {
            type Output = crate::expr::num::arith::DivExpr;

            fn div(self, rhs: R) -> Self::Output {
                crate::expr::num::arith::DivExpr::new(Box::new(self), Box::new(rhs))
            }
        }

        impl<R: Numeric + 'static> std::ops::Rem<R> for $struct {
            type Output = crate::expr::num::arith::RemExpr;

            fn rem(self, rhs: R) -> Self::Output {
                crate::expr::num::arith::RemExpr::new(Box::new(self), Box::new(rhs))
            }
        }
    };
}
pub(super) use impl_arith_ops;

impl_arith_ops!(AddExpr);
impl_arith_ops!(SubExpr);
impl_arith_ops!(MulExpr);
impl_arith_ops!(DivExpr);
impl_arith_ops!(RemExpr);
impl_arith_ops!(Column);
