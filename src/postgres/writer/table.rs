use crate::postgres::def::TableDef;
use sea_query::{Alias, Iden, Table, TableStatement};

impl TableDef {
    pub fn write(&self) -> TableStatement {
        let mut table = Table::create();
        table.table(Alias::new(self.info.name.as_ref()));
        for col in self.columns.iter() {
            table.col(col.write());
        }
        // table.engine(self.info.engine.to_string().as_str());
        // table.character_set(self.info.char_set.to_string().as_str());
        // table.collate(self.info.collation.to_string().as_str());
        // for idx in self.indexes.iter() {
        //     table.index(idx.write());
        // }
        // for key in self.foreign_keys.iter() {
        //     table.foreign_key(key.write());
        // }
        TableStatement::Create(table)
    }
}
