pub use crate::expr::prelude::*;

pub mod and;
pub use and::And;

pub mod or;
pub use or::Or;

pub mod not;
pub use not::Not;

impl<L, R> std::ops::BitAnd<CommonExpr<R>> for CommonExpr<L>
where
    L: Boolean + 'static,
    R: Boolean + 'static,
{
    type Output = CommonExpr<And>;

    fn bitand(self, rhs: CommonExpr<R>) -> Self::Output {
        CommonExpr(And::new(Box::new(self), Box::new(rhs)))
    }
}

impl<L, R> std::ops::BitOr<CommonExpr<R>> for CommonExpr<L>
where
    L: Boolean + 'static,
    R: Boolean + 'static,
{
    type Output = CommonExpr<Or>;

    fn bitor(self, rhs: CommonExpr<R>) -> Self::Output {
        CommonExpr(Or::new(Box::new(self), Box::new(rhs)))
    }
}

impl<T> std::ops::Not for CommonExpr<T>
where
    T: Boolean + 'static,
{
    type Output = CommonExpr<Not>;

    fn not(self) -> Self::Output {
        CommonExpr(Not::new(Box::new(self)))
    }
}
