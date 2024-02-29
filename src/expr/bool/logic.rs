pub mod and;
pub use and::*;

pub mod or;
pub use or::*;

pub mod not;
pub use not::*;


/// Implement `BitAnd`, `BitOr`, and `Not` as `AND`, `OR`, and `NOT` operations respectively.
macro_rules! impl_bool_logic {
    ($struct:ident) => {
        impl<R: Boolean + 'static> std::ops::BitAnd<R> for $struct {
            type Output = $crate::expr::bool::logic::and::AndExpr;

            fn bitand(self, rhs: R) -> Self::Output {
                $crate::expr::bool::logic::and::AndExpr::new(Box::new(self), Box::new(rhs))
            }
        }

        impl<R: Boolean + 'static> std::ops::BitOr<R> for $struct {
            type Output = $crate::expr::bool::logic::or::OrExpr;

            fn bitor(self, rhs: R) -> Self::Output {
                $crate::expr::bool::logic::or::OrExpr::new(Box::new(self), Box::new(rhs))
            }
        }

        impl std::ops::Not for $struct {
            type Output = $crate::expr::bool::logic::not::NotExpr;

            fn not(self) -> Self::Output {
                $crate::expr::bool::logic::not::Not::not(self)
            }
        }
    };
}
pub(in super::super) use impl_bool_logic;
