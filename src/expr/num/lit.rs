use crate::expr::prelude::*;

macro_rules! impl_numeric_lit {
    ($t:ty) => {
        impl Client for $t {
            type Ctx = ExprType;
            type Msg = Message;

            fn children(
                &self,
                ctx: Self::Ctx,
            ) -> Vec<(&dyn Client<Ctx = Self::Ctx, Msg = Self::Msg>, Self::Ctx)> {
                Vec::new()
            }

            fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg> {
                Vec::new()
            }

            fn send_all(&self, ctx: Self::Ctx, server: &mut dyn Server<Msg = Self::Msg>) {}
        }
        impl Checkable for $t {}
        impl Expression for $t {
            fn eval_type(&self) -> ExprType {
                ExprType::Num
            }

            fn display(&self, dialect: Dialect) -> String {
                self.to_string()
            }
        }
        impl Common for $t {}
        impl Numeric for $t {}
    };
}

impl_numeric_lit!(u8);
impl_numeric_lit!(u16);
impl_numeric_lit!(u32);
impl_numeric_lit!(u64);
impl_numeric_lit!(u128);
impl_numeric_lit!(usize);

impl_numeric_lit!(i8);
impl_numeric_lit!(i16);
impl_numeric_lit!(i32);
impl_numeric_lit!(i64);
impl_numeric_lit!(i128);
impl_numeric_lit!(isize);
