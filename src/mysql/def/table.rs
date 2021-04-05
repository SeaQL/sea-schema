use super::Collation;

#[derive(Debug, PartialEq)]
pub struct TableInfo {
    /// The name of the table
    pub name: String,
    pub engine: String,
    pub auto_increment: u64,
    pub collation: Collation,
    pub comment: String,
}