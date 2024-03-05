//! Expressions that evaluate into custom types

#[macro_use]
pub mod multi;
#[doc(inline)]
pub use multi::{IntoMulti, IntoMultiCore, IntoMultiMisc, MultiExpr};

pub mod alias;
#[doc(inline)]
pub use alias::AliasExpr;
