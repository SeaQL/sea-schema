use super::{IndexInfo, TableDef};

#[derive(Clone, Debug)]
pub struct Schema {
    pub tables: Vec<TableDef>,
    pub indexes: Vec<IndexInfo>,
}

impl Schema {
    pub fn collect_unique_indexes(mut self) -> Self {
        for table in self.tables.iter_mut() {
            for index in self.indexes.iter() {
                if index.unique && index.table_name == table.name {
                    table.constraints.push(index.clone());
                }
            }
        }
        self
    }
}
