use crate::expr::prelude::*;

// these cases are more complex
// and `neq` is likely to deviate when display is further refined
// so they are kept macro-less despite their repetition
pub mod eq;
pub use eq::*;

pub mod neq;
pub use neq::*;

/// Defines and implements behaviour for the 4 basic comparison operators: `<`, `<=`, `>`, `>=`.
///
/// Arguments are ($struct, $trait, $method, $display)
macro_rules! impl_num_cmp {
    ($struct:ident, $trait:ident, $method:ident, $display:expr) => {
        #[derive(IntoMultiCore)]
        pub struct $struct {
            lhs: Box<dyn Numeric>,
            rhs: Box<dyn Numeric>,
        }

        impl Expression for $struct {
            fn conditions(&self, coerce: ExprType) -> Box<dyn Iterator<Item = Condition> + '_> {
                debug_assert!(matches!(coerce, ExprType::Any | ExprType::Bool));

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
                ExprType::Bool
            }
        }
        impl Boolean for $struct {}

        // Implement `AND` and `OR` via `BitAnd` and `BitOr`
        super::logic::impl_bool_logic!($struct);

        pub trait $trait<R> {
            fn $method(self, rhs: R) -> $struct;
        }

        impl<L, R> $trait<R> for L
        where
            L: Numeric + 'static,
            R: Numeric + 'static,
        {
            fn $method(self, rhs: R) -> $struct {
                $struct {
                    lhs: Box::new(self),
                    rhs: Box::new(rhs),
                }
            }
        }
    };
}

impl_num_cmp!(GTExpr, GT, gt, "{} > {}");
impl_num_cmp!(GEqExpr, GEq, geq, "{} >= {}");
impl_num_cmp!(LTExpr, LT, lt, "{} < {}");
impl_num_cmp!(LEqExpr, LEq, leq, "{} <= {}");
