use crate::postgres::def::TableDef;
use sea_query::{Alias, Iden, Table, TableStatement};

impl TableDef {
    pub fn write(&self) -> TableStatement {
        let mut table = Table::create();
        table.table(Alias::new(self.info.name.as_ref()));
        for col in self.columns.iter() {
            table.col(&mut col.write());
        }
        for primary_key in self.primary_key_constraints.iter() {
            table.primary_key(&mut primary_key.write());
        }
        for unique in self.unique_constraints.iter() {
            table.index(&mut unique.write());
        }
        for reference in self.reference_constraints.iter() {
            table.foreign_key(&mut reference.write());
        }
        TableStatement::Create(table)
    }
}
