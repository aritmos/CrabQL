use super::value;

#[derive(Default)]
pub struct Expr;

impl Expr {
    pub fn new() -> Self {
        Self {}
    }
}

impl std::ops::BitAnd for Expr {
    type Output = Self;

    /// Joins two expressions via a logical _AND_.
    /// Equivalent to using the `.and()` method.
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitOr for Expr {
    type Output = Self;

    /// Joins two expressions via a logical _OR_.
    /// Equivalent to using the `.or()` method.
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl Expr {
    pub fn and(self, _rhs: Self) -> Self {
        todo!()
    }

    pub fn or(self, _rhs: Self) -> Self {
        todo!()
    }
}

impl value::Comparison for Expr {
    type Expr = Self;

    fn eq(&self, _val: impl Into<value::Value>) -> Self::Expr {
        todo!()
    }

    fn ne(&self, _val: impl Into<value::Value>) -> Self::Expr {
        todo!()
    }

    fn gt(&self, _val: impl Into<value::Value>) -> Self::Expr {
        todo!()
    }

    fn ge(&self, _val: impl Into<value::Value>) -> Self::Expr {
        todo!()
    }

    fn lt(&self, _val: impl Into<value::Value>) -> Self::Expr {
        todo!()
    }

    fn le(&self, _val: impl Into<value::Value>) -> Self::Expr {
        todo!()
    }
}
