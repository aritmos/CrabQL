use super::{
    expr,
    funcs::Functions,
    value::{Comparison, Value},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Column<'c> {
    pub(super) name: &'c str,
}

impl<'c> Column<'c> {
    pub(super) fn new(name: &'c str) -> Self {
        Self { name }
    }
}

impl<'c> Comparison for Column<'c> {
    type Expr = expr::Expr;

    fn eq(&self, _val: impl Into<Value>) -> Self::Expr {
        todo!()
    }

    fn ne(&self, _val: impl Into<Value>) -> Self::Expr {
        todo!()
    }

    fn gt(&self, _val: impl Into<Value>) -> Self::Expr {
        todo!()
    }

    fn ge(&self, _val: impl Into<Value>) -> Self::Expr {
        todo!()
    }

    fn lt(&self, _val: impl Into<Value>) -> Self::Expr {
        todo!()
    }

    fn le(&self, _val: impl Into<Value>) -> Self::Expr {
        todo!()
    }
}

impl<'c> Functions for Column<'c> {
    type Expr = expr::Expr;

    fn len(self) -> Self::Expr {
        todo!()
    }
}

impl<'c> IntoIterator for Column<'c> {
    type Item = Column<'c>;

    type IntoIter = std::iter::Once<Self>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}
