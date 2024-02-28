#![allow(unused)]
#![feature(trait_upcasting)]

/// Expressions and Functions
pub mod expr;

/// Create SQL Expressions
pub mod reader;

/// Query Verification
pub mod checker;

/// Experimental Testing
pub mod testing;

/// Schemas
pub mod schema;

// #[cfg(test)]
// mod tests {
//     use super::super::{checker::DerivedChecker, expr::funcs::Mappings};
//     use super::*; // Required for things to be in scope
//
//     #[test]
//     fn reader_syntax() {
//         let mut checker = DerivedChecker::new();
//         let _x = Reader::<'_, _, usize, usize>::new(&mut checker)
//             .table("emp")
//             .filter(|t| t["id"].gt(3_usize) & t["name"].len().gt(10_usize))
//             .select(|t| [t["id"], t["name"]]);
//     }
// }
