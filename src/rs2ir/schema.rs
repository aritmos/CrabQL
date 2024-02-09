use super::{
    checker::{CheckResult, Checker},
    read::ReaderT,
    write::WriterT,
};

/// A complete schema provided by the user.
pub struct CompiledSchema;

/// A blank schema that is built upon use.
#[derive(Default)]
pub struct DerivedSchema;

impl DerivedSchema {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Schema: Sized + Checker {
    fn read(&mut self) -> ReaderT<Self> {
        ReaderT::new(self, 0, Ok(()))
    }

    fn write(&mut self) -> WriterT<Self> {
        WriterT::new(self, 0, Ok(()))
    }
}

impl Checker for DerivedSchema {
    type Expr = ();

    unsafe fn check(&self, _expr: Self::Expr) -> CheckResult {
        todo!()
    }

    unsafe fn merge(&mut self, _expr: Self::Expr) {
        todo!()
    }
}

impl Schema for DerivedSchema {
    fn read(&mut self) -> ReaderT<Self> {
        ReaderT::new(self, 0, Ok(()))
    }

    fn write(&mut self) -> WriterT<Self> {
        WriterT::new(self, 0, Ok(()))
    }
}
