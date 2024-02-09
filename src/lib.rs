/// Rust to Intermediate Representation
pub mod rs2ir;

/// Intermediate Representation to Rust
pub mod ir2sql;

/// SQL to Intermediate Representation
pub mod sql2ir;

pub mod prelude {
    pub use crate::rs2ir::funcs::Functions;
}
