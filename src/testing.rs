#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExprType {
    Any,
    Num,
    Bool,
}
pub enum Dialect {}
pub enum Condition {
    /// A column with the given `id` exists in context and has the given type
    // RFC: refactor into proper type?
    Col(String, ExprType),
}
use std::{cell::RefCell, rc::Rc};
#[derive(Clone)]
pub struct Ctx {
    // shared information (immutable during `check`)
    // - e.g. current table selection
    tables: Rc<RefCell<() /* <- replace with some type */>>,
    // individual information (overridable during `check`)
    // - e.g. expression coercion
    pub coerce: ExprType,
}
impl Ctx {
    // Returns a clone of self with the given coercion
    pub fn set_coerce(&self, coerce: ExprType) -> Self {
        let mut ctx = self.clone();
        ctx.coerce = coerce;
        ctx
    }
}

pub struct ExprInfo {
    // e.g.
    // - used column ids to know when to require a subquery
    cols: Vec<String>,
}
pub enum ExprErr {
    UnSealed(usize),
    Sealed(usize),
}

pub trait Checker {
    type Cond;
    type Ctx;
    fn validate(&mut self, cond: Self::Cond, ctx: Self::Ctx);
    fn flush(&mut self) -> Result<ExprInfo, ExprErr>;
}

pub struct CompiledChecker {
    state: Result<ExprInfo, ExprErr>,
}
impl Checker for CompiledChecker {
    type Cond = Condition;

    type Ctx = ();

    fn validate(&mut self, cond: Self::Cond, ctx: Self::Ctx) {
        match self.state {
            Ok(_) => (),
            Err(ExprErr::Sealed(_)) => return,
            Err(ExprErr::UnSealed(_)) => todo!(),
        }
        todo!()
    }

    fn flush(&mut self) -> Result<ExprInfo, ExprErr> {
        todo!()
    }
}

// A type that can be passed into a `Checker<Cond, Ctx>`
pub trait Checkable {
    type Cond;
    type Ctx: Clone;

    // returns references to children objects that need to be checked
    // (children can be of different types and might require other contexts)
    #[allow(clippy::type_complexity)] // no way around it
    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(
        &dyn Checkable<Cond = Self::Cond, Ctx = Self::Ctx>,
        Self::Ctx,
    )>;

    // returns the conditions needed to verify itself assuming children have already been checked
    fn conditions(&self, ctx: Self::Ctx) -> Vec<Self::Cond>;

    // checker is not an associated type as we want multiple checkers for the same expr
    fn check(&self, checker: &mut dyn Checker<Cond = Self::Cond, Ctx = Self::Ctx>, ctx: Self::Ctx) {
        // check children
        for (child, ctx) in self.children(ctx.clone()) {
            // if coerce is inside ctx, then each ctx needs to be possibly mutated per child
            child.check(checker, ctx);
        }

        // check self
        for cond in self.conditions(ctx.clone()) {
            checker.validate(cond, ctx.clone());
        }
    }
}

pub trait Expression: Checkable<Cond = Condition, Ctx = Ctx> {
    fn display(&self, dialect: Dialect) -> String;
}

pub trait CoreExpression: Expression {
    fn eval_type(&self) -> ExprType;
}
pub trait Numeric: CoreExpression {}
pub trait Boolean: CoreExpression {}
pub trait MiscExpression: Expression {}

pub struct Builder<Chckr> {
    checker: Chckr,
}

// Faciliate CoreExpression coercion verification
macro_rules! debug_assert_coerce {
    ($self:expr, $ctx:expr) => {
        debug_assert!({ $ctx.coerce == ExprType::Any || $ctx.coerce == $self.eval_type() })
    };
}

pub struct GtExpr {
    lhs: Box<dyn Numeric>,
    rhs: Box<dyn Numeric>,
}
impl Checkable for GtExpr {
    type Cond = Condition;

    type Ctx = Ctx;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(
        &dyn Checkable<Cond = Self::Cond, Ctx = Self::Ctx>,
        Self::Ctx,
    )> {
        vec![
            (self.lhs.as_ref(), ctx.set_coerce(ExprType::Num)),
            (self.rhs.as_ref(), ctx.set_coerce(ExprType::Num)),
        ]
    }

    fn conditions(&self, ctx: Self::Ctx) -> Vec<Self::Cond> {
        debug_assert_coerce!(self, ctx);
        // no self conditions
        Vec::new()
    }
}

impl Expression for GtExpr {
    fn display(&self, dialect: Dialect) -> String {
        todo!()
    }
}
impl CoreExpression for GtExpr {
    fn eval_type(&self) -> ExprType {
        ExprType::Bool
    }
}

pub struct Column {
    id: String,
}
impl Checkable for Column {
    type Cond = Condition;
    type Ctx = Ctx;

    fn children(
        &self,
        ctx: Self::Ctx,
    ) -> Vec<(
        &dyn Checkable<Cond = Self::Cond, Ctx = Self::Ctx>,
        Self::Ctx,
    )> {
        // no children
        Vec::new()
    }

    fn conditions(&self, ctx: Self::Ctx) -> Vec<Self::Cond> {
        // exists and is type
        vec![Condition::Col(self.id.clone(), ExprType::Num)]
    }
}

impl Expression for Column {
    fn display(&self, dialect: Dialect) -> String {
        self.id.clone()
    }
}
