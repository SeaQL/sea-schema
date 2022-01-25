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
use sea_query::TableCreateStatement;

impl Schema {
    pub fn write(&self) -> Vec<TableCreateStatement> {
        self.tables.iter().map(|table| table.write()).collect()
    }
}
