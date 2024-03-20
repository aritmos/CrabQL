use crate::expr::ExprType;

/// The conditions that the standard [`crate::checker::Checker`]s verify
#[derive(Debug)]
pub enum Condition {
    ColExists(String),
    ColExistsAndType(String, ExprType),
    CaseRetErr((ExprType, ExprType)),
}
