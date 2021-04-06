#[derive(Clone, Debug, PartialEq)]
pub struct ForeignKeyInfo {
    /// The name of the foreign key
    pub name: String,
    /// The columns composing this foreign key
    pub columns: Vec<String>,
    /// Referenced table name
    pub referenced_table: String,
    /// The columns composing the index of the referenced table
    pub referenced_columns: Vec<String>,
    /// Action on update
    pub on_update: ForeignKeyAction,
    /// Action on delete
    pub on_delete: ForeignKeyAction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForeignKeyAction {
    Cascade,
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
}