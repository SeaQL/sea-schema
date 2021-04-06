use super::{Collation, StorageEngine};

#[derive(Clone, Debug, PartialEq)]
pub struct TableInfo {
    /// The name of the table
    pub name: String,
    pub engine: StorageEngine,
    pub auto_increment: Option<u64>,
    pub collation: Collation,
    pub comment: String,
}
