pub trait Expression {}
pub trait CoreExpression {}
pub trait MiscExpression {}

pub struct MultiExpr<T> {
    exprs: Vec<T>,
}

impl Expression for MultiExpr<Box<dyn CoreExpression>> {}
impl CoreExpression for MultiExpr<Box<dyn CoreExpression>> {}

impl<T: MiscExpression> Expression for MultiExpr<T> {}
impl<T: MiscExpression> MiscExpression for MultiExpr<T> {}
