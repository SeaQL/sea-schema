//! To write [`mysql::Schema`] to SQL statements

mod types;

pub use types::*;

use sea_query::SchemaStatement;
use super::def::Schema;

impl Schema {
    pub fn build() -> Vec<SchemaStatement> {
        vec![]
    }
}