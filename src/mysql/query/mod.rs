//! To support querying MySQL's INFORMATION_SCHEMA

pub struct SchemaQuery;

mod column;
mod index;

pub use column::*;
pub use index::*;