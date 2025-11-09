use super::{IndexInfo, TableDef};

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub tables: Vec<TableDef>,
    pub indexes: Vec<IndexInfo>,
}

impl Schema {
    pub fn merge_indexes_into_table(mut self) -> Self {
        for table in self.tables.iter_mut() {
            for index in self.indexes.iter() {
                if index.table_name == table.name {
                    table.constraints.push(index.clone());
                }
            }
        }
        self
    }
}
