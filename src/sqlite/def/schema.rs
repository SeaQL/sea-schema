use super::TableDef;

#[derive(Clone, Debug)]
pub struct Schema {
    pub tables: Vec<TableDef>,
}
