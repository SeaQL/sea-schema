#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/infoschema-table-constraints.html
pub enum TableConstraintsField {
    ConstraintSchema,
    ConstraintName,
    TableSchema,
    TableName,
    ConstraintType,
    IsDeferrable,
    InitiallyDeferred,
}
