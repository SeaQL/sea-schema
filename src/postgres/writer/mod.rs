mod column;
mod constraints;
mod enumeration;
mod schema;
mod table;
mod types;

pub use column::*;
pub use constraints::*;
pub use enumeration::*;
pub use schema::*;
pub use table::*;
pub use types::*;

use super::def::Schema;
use sea_query::SchemaStatement;

impl Schema {
    pub fn write(&self) -> Vec<SchemaStatement> {
        let mut statements = Vec::new();
        for table in self.tables.iter() {
            statements.push(SchemaStatement::TableStatement(table.write()));
        }
        statements
    }
}
