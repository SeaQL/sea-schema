use super::{Collation, StorageEngine};

#[derive(Clone, Debug, PartialEq)]
pub struct TableInfo {
    /// The name of the table
    pub name: String,
    pub engine: String,
    pub auto_increment: StorageEngine,
    pub collation: Collation,
    pub comment: String,
}
