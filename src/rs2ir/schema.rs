/// Compiled schema
pub struct CompiledSchema;

/// Derived Schema
#[derive(Default)]
pub struct DerivedSchema;

impl DerivedSchema {
    pub fn new() -> Self {
        Self {}
    }
}
