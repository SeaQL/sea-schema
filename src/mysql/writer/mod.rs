//! To write [`mysql::Schema`] to SQL statements

mod column;
mod index;
mod table;
mod types;

pub use column::*;
pub use index::*;
pub use table::*;
pub use types::*;

use sea_query::SchemaStatement;
use super::def::Schema;

impl Schema {
    pub fn write(&self) -> Vec<SchemaStatement> {
        let mut statements = Vec::new();
        for table in self.tables.iter() {
            statements.push(SchemaStatement::TableStatement(table.write()));
        }
        statements
    }
}
