pub enum Value {
    Int(isize),
    UInt(usize),
    Str(String),
    Null,
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Str(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::Int(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::UInt(value)
    }
}

pub trait Comparison {
    type Expr;

    fn eq(&self, val: impl Into<Value>) -> Self::Expr;

    fn ne(&self, val: impl Into<Value>) -> Self::Expr;

    fn gt(&self, val: impl Into<Value>) -> Self::Expr;

    fn ge(&self, val: impl Into<Value>) -> Self::Expr;

    fn lt(&self, val: impl Into<Value>) -> Self::Expr;

    fn le(&self, val: impl Into<Value>) -> Self::Expr;
}
