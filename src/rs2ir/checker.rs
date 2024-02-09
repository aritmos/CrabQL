pub type CheckOk = ();
pub type CheckErr = String;
pub type CheckResult = Result<CheckOk, CheckErr>;
///
/// Used to check and
pub trait Checker {
    type Expr;

    /// Checks if an expression is allowed
    /// # Safety
    /// This function meant to be used immediately prior to `Checker::update` with the same `Expr`.
    /// The caller guarantees that no other merge operation occurs in between these two calls.
    /// Consider using the bundled `Self::check_and_update` method instead.
    unsafe fn check(&self, expr: Self::Expr) -> CheckResult;

    /// Updates itself (if required) given an expression
    /// # Safety
    /// See safety for `Checker::check`.
    unsafe fn update(&mut self, expr: Self::Expr);

    /// Checks an expression; if it succeeds it potentially merges it.
    fn check_and_update(&mut self, expr: Self::Expr) -> CheckResult
    where
        Self::Expr: Copy, // TODO: We could use references but we probably want to pass the
                          // Expr into inner anyways which would require some form of a copy
    {
        // Safety: We are calling the functions sequentially as required.
        unsafe {
            let expr_ok = self.check(expr);
            if expr_ok.is_ok() {
                self.update(expr);
            }
            expr_ok
        }
    }
}
