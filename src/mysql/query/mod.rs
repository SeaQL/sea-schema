//! To support querying MySQL's INFORMATION_SCHEMA

use crate::mysql::def::Version;

#[derive(Debug, Default)]
pub struct SchemaQuery {
    version: Version,
}

mod column;
mod constraint;
mod index;
mod version;

pub use column::*;
pub use constraint::*;
pub use index::*;
pub use version::*;

impl SchemaQuery {
    pub fn new(version: Version) -> Self {
        Self {
            version
        }
    }
}