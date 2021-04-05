#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-key-column-usage-table.html
pub enum KeyColumnUsageFields {
    ConstraintSchema,
    ConstraintName,
    TableSchema,
    TableName,
    ColumnName,
    OrdinalPosition,
    PositionInUniqueConstraint,
    ReferencedTableSchema,
    ReferencedTableName,
    ReferencedColumnName,
}

#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema-referential-constraints-table.html
pub enum ReferentialConstraintsFields {
    ConstraintSchema,
    ConstraintName,
    UniqueConstraintSchema,
    UniqueConstraintName,
    UpdateRule,
    DeleteRule,
    TableName,
    ReferencedTableName,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum ForeignKeyAction {
    Cascade,
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
}