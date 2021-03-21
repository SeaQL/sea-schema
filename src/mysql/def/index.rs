#[derive(Debug, sea_query::Iden)]
pub enum StatisticsFields {
    TableCatalog,
    TableSchema,
    TableName,
    NonUnique,
    IndexSchema,
    IndexName,
    SeqInIndex,
    ColumnName,
    Collation,
    Cardinality,
    SubPart,
    Packed,
    Nullable,
    IndexType,
    Comment,
    IndexComment,
    IsVisible,
    Expression,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum IndexOrder {
    Ascending,
    Descending,
    Unordered,
}

#[derive(Debug, PartialEq)]
pub enum IndexType {
    BTree,
    FullText,
    Hash,
    RTree,
    Spatial,
}