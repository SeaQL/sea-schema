#[cfg(feature="with-serde")] use serde::{Serialize, Deserialize};

use crate as sea_schema;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct IndexInfo {
    /// Does this index requires unique values
    pub unique: bool,
    /// The name of the index
    pub name: String,
    /// The columns composing this index. If `functional` is true, it may contains expressions
    pub columns: Vec<String>,
    /// Ascending, descending or unordered
    pub order: IndexOrder,
    /// If the whole column is indexed, this value is null. Otherwise the number indicates number of characters indexed
    pub sub_part: Option<u32>,
    /// Does this index allow null values
    pub nullable: bool,
    /// BTree (the default), full-text etc
    pub idx_type: IndexType,
    /// User comments
    pub comment: String,
    /// True if part of the index is computed
    pub functional: bool,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum IndexOrder {
    Ascending,
    Descending,
    Unordered,
}

#[derive(Clone, Debug, PartialEq, sea_schema_derive::Name)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum IndexType {
    #[name = "BTREE"] BTree,
    #[name = "FULLTEXT"] FullText,
    #[name = "HASH"] Hash,
    #[name = "RTREE"] RTree,
    #[name = "SPATIAL"] Spatial,
}