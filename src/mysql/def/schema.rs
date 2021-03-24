#[derive(Debug, sea_query::Iden)]
/// Ref: https://dev.mysql.com/doc/refman/8.0/en/information-schema.html
pub enum InformationSchema {
    #[iden = "information_schema"]
    Schema,
    Columns,
    Statistics,
    KeyColumnUsage,
    ReferentialConstraints,
}
