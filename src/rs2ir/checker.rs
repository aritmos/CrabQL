use super::{
    expr::ExprResult,
    schema::{CompiledSchema, DerivedSchema},
};

pub type CheckOk = ();
pub enum CheckErr {}
pub type CheckResult = Result<CheckOk, CheckErr>;

pub struct CompiledChecker<'s> {
    schema: &'s CompiledSchema,
    // "virtual schema" goes here
}

pub struct DerivedChecker {
    schema: DerivedSchema,
}

impl DerivedChecker {
    pub fn new() -> Self {
        Self {
            schema: DerivedSchema::new(),
        }
    }
}

/// Used to check and merge expressions.
pub trait Checker {
    type Expr;

    /// Checks if an expression is allowed
    /// # Safety
    /// This function meant to be used immediately prior to `Checker::merge` with the same `expr`.
    /// The caller guarantees that no other merge operation occurs in between these two calls.
    /// Consider using the bundled `Self::check_and_update` method instead.
    unsafe fn check(&self, expr: Self::Expr) -> CheckResult;

    /// Possibly merges the expression into self.
    /// # Safety
    /// See safety for `Checker::check`.
    unsafe fn merge(&mut self, expr: Self::Expr);

    /// Checks an expression; if it succeeds it potentially merges it.
    fn check_and_merge(&mut self, expr: Self::Expr) -> CheckResult
    where
        Self::Expr: Copy, // TODO: We could use references but we probably want to pass the
                          // Expr into inner anyways which would require some form of a copy
    {
        // Safety: We are calling the functions sequentially as required.
        unsafe {
            let expr_ok = self.check(expr);
            if expr_ok.is_ok() {
                self.merge(expr);
            }
            expr_ok
        }
    }
}

impl Checker for DerivedChecker {
    type Expr = ExprResult;

    unsafe fn check(&self, _expr: Self::Expr) -> CheckResult {
        todo!()
    }

    unsafe fn merge(&mut self, _expr: Self::Expr) {
        todo!()
    }
}

impl<'s> Checker for CompiledChecker<'s> {
    type Expr = ExprResult;

    unsafe fn check(&self, _expr: Self::Expr) -> CheckResult {
        todo!()
    }

    unsafe fn merge(&mut self, _expr: Self::Expr) {
        todo!()
    }
}
