//! Arithmetic operators as expressions

use crate::expr::prelude::*;

macro_rules! impl_arith_expr {
    ($struct:ident, $display:expr) => {
        pub struct $struct {
            lhs: Box<dyn Expression>, // Numeric
            rhs: Box<dyn Expression>, // Numeric
        }

        impl $struct {
            pub fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
                Self { lhs, rhs }
            }
        }

        impl Client for $struct {
            type Ctx = ExprType;
            type Msg = Message;

            fn children(
                &self,
                ctx: Self::Ctx,
            ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
                vec![
                    (self.lhs.as_ref(), ExprType::Num),
                    (self.rhs.as_ref(), ExprType::Num),
                ]
            }

            fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
                Vec::new()
            }
        }
        impl Checkable for $struct {}
        impl Expression for $struct {
            fn eval_type(&self) -> ExprType {
                ExprType::Num
            }

            fn display(&self, dialect: Dialect) -> String {
                format!(
                    $display,
                    self.lhs.display(dialect),
                    self.rhs.display(dialect)
                )
            }
        }
        impl Common for $struct {}
        impl Numeric for $struct {}
    };
}

impl_arith_expr!(Add, "({} + {})");
impl_arith_expr!(Sub, "({} - {})");
impl_arith_expr!(Mul, "({} * {})");
impl_arith_expr!(Div, "({} / {})");
impl_arith_expr!(Rem, "({} % {})");

impl<L, R> std::ops::Add<R> for CommonExpr<L>
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    type Output = CommonExpr<Add>;

    fn add(self, rhs: R) -> Self::Output {
        super::Numeric::add(self, rhs)
    }
}

impl<L, R> std::ops::Sub<R> for CommonExpr<L>
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    type Output = CommonExpr<Sub>;

    fn sub(self, rhs: R) -> Self::Output {
        super::Numeric::sub(self, rhs)
    }
}

impl<L, R> std::ops::Mul<R> for CommonExpr<L>
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    type Output = CommonExpr<Mul>;

    fn mul(self, rhs: R) -> Self::Output {
        super::Numeric::mul(self, rhs)
    }
}

impl<L, R> std::ops::Div<R> for CommonExpr<L>
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    type Output = CommonExpr<Div>;

    fn div(self, rhs: R) -> Self::Output {
        super::Numeric::div(self, rhs)
    }
}

impl<L, R> std::ops::Rem<R> for CommonExpr<L>
where
    L: Numeric + 'static,
    R: Numeric + 'static,
{
    type Output = CommonExpr<Rem>;

    fn rem(self, rhs: R) -> Self::Output {
        super::Numeric::rem(self, rhs)
    }
}
