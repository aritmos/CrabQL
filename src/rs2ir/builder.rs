use super::checker::Checker;

/// TODO: Common functionality that we want to enforce between Readers and Writers
pub trait Builder: Checker {}

// pub trait BuilderError {}
//
// impl Builder {
//     pub fn is_ok(&self) -> Result<(), impl BuilderError> {
//         todo!()
//     }
// }

// pub enum BuilderError {
//     TableNotFound,
//     ColumnNotFound,
//     IllegalOperation,
// }
