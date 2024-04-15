use crate::expr::ExprType;

/// The conditions that the standard [`crate::checker::Checker`]s verify
#[derive(Debug)]
pub enum Condition {
    /// Does a column exist in this context
    ColExists(String),
    /// Does a column exist in this context, and does it have this type?
    ColExistsAndType(String, ExprType),
}

impl From<Condition> for Message {
    fn from(cond: Condition) -> Self {
        Message::Cond(cond)
    }
}

#[derive(Debug)]
pub enum Signal {
    /// Signifies the start of a linking process
    StartLink,
    /// Signifies the end of a linking process
    EndLink,
    /// Mismatch of CommonExpression types
    /// (Expected, Found)
    TypeMismatch((ExprType, ExprType)),
}

impl From<Signal> for Message {
    fn from(sig: Signal) -> Self {
        Message::Sig(sig)
    }
}

#[derive(Debug)]
pub enum Message {
    Cond(Condition),
    Sig(Signal),
}
