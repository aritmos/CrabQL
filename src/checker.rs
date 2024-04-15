/// Standard conditions for a checker
mod message;
pub use message::{Condition, Message, Signal};

use crate::expr::ExprType;
use crate::protocol::{Client, Server, ServerHandler};
use crate::schema::{CompiledSchema, DerivedSchema};

pub type Context = ExprType;

/// A type that can check `Checkable` types.
pub trait Checker: ServerHandler + Server<Msg = Message> {}

/// A type that can be checked by a `Checker`.
pub trait Checkable: Client<Ctx = Context, Msg = Message> {}

// === Concrete Impls ===

pub struct CompiledChecker<'s> {
    schema: &'s CompiledSchema,
    state: (),
    // another field should go here to track user additions
    // TODO
}

#[derive(Default)]
pub struct DerivedChecker {
    schema: DerivedSchema,
    state: (),
}

impl DerivedChecker {
    pub fn new() -> Self {
        Self {
            schema: DerivedSchema::new(),
            state: (),
        }
    }
}

impl Server for DerivedChecker {
    type Msg = Message;

    fn accept(&mut self, msg: Self::Msg) {
        todo!()
    }
}
impl ServerHandler for DerivedChecker {
    fn state(&self) -> Result<(), ()> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
impl Checker for DerivedChecker {}

impl<'s> Server for CompiledChecker<'s> {
    type Msg = Message;

    fn accept(&mut self, msg: Self::Msg) {
        todo!()
    }
}
impl<'s> ServerHandler for CompiledChecker<'s> {
    fn state(&self) -> Result<(), ()> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
impl<'s> Checker for CompiledChecker<'s> {}
