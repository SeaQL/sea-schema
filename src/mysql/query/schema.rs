use crate::mysql::def::Version;

#[derive(Debug, Default)]
pub struct SchemaQuery {
    pub version: Version,
}

impl SchemaQuery {
    pub fn new(version: Version) -> Self {
        Self {
            version
        }
    }
}

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