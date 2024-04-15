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
                ExprType::Bool
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
        impl Boolean for $struct {}
    };
}

impl_num_cmp!(Gt, "{} > {}");
impl_num_cmp!(Geq, "{} >= {}");
impl_num_cmp!(Lt, "{} < {}");
impl_num_cmp!(Leq, "{} <= {}");
