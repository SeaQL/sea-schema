#[derive(Debug, Default)]
pub struct SchemaQueryBuilder;

#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/information-schema.html
pub enum InformationSchema {
    #[iden = "information_schema"]
    Schema,
    Columns,
    CheckConstraints,
    KeyColumnUsage,
    ReferentialConstraints,
    Tables,
    TableConstraints,
    ConstraintColumnUsage,
}
