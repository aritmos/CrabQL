use crate::expr::ExprType;

/// The conditions that the standard [`crate::checker::Checker`]s verify
pub enum Condition {
    ColExists(String),
    ColIsType(ExprType),
}
