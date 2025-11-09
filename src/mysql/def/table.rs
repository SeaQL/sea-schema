#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use super::{CharSet, Collation, StorageEngine};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct TableInfo {
    /// The name of the table
    pub name: String,
    pub engine: StorageEngine,
    pub auto_increment: Option<u64>,
    pub char_set: Option<CharSet>,
    pub collation: Option<Collation>,
    pub comment: String,
}
