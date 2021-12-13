use crate::sqlite::TableDef;

#[derive(Clone, Debug)]
pub struct Schema {
    pub tables: Vec<TableDef>,
}
