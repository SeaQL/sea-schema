#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Schema {
    pub schema: String,
    pub tables: Vec<TableDef>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct TableDef {
    pub info: TableInfo,
    pub columns: Vec<ColumnInfo>,
    pub constraints: Vec<Constraint>,
    pub of_type: Option<Type>,
    pub inherets: Vec<String>,
}
