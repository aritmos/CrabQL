/// Standard conditions for a checker
mod cond;

use super::{
    expr::ExprResult,
    reader::schema::{CompiledSchema, DerivedSchema},
};

pub type CheckOk = ();
pub enum CheckErr {}
pub type CheckResult = Result<CheckOk, CheckErr>;

pub struct CompiledChecker<'s> {
    schema: &'s CompiledSchema,
    // another field should go here to track user additions
    // TODO
}

#[derive(Default)]
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
    type Cond;
    type Ctx;

    fn validate(
        &mut self,
        ctx: &Self::Ctx,
        conditions: Box<dyn Iterator<Item = Self::Cond>>,
    ) -> CheckResult;
}

impl Checker for DerivedChecker {
    type Cond = cond::Condition;

    // TODO: Update when context is implemented
    type Ctx = ();

    fn validate(
        &mut self,
        ctx: &Self::Ctx,
        conditions: Box<dyn Iterator<Item = Self::Cond>>,
    ) -> CheckResult {
        todo!()
    }
}

impl<'s> Checker for CompiledChecker<'s> {
    type Cond = cond::Condition;

    // TODO: Update when context is implemented
    type Ctx = ();

    fn validate(
        &mut self,
        ctx: &Self::Ctx,
        conditions: Box<dyn Iterator<Item = Self::Cond>>,
    ) -> CheckResult {
        todo!()
    }
}
