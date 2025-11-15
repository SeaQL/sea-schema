#[derive(Debug, sea_query::Iden)]
/// Ref: https://www.postgresql.org/docs/13/infoschema-referential-constraints.html
pub enum ReferentialConstraintsFields {
    ConstraintSchema,
    ConstraintName,
    UniqueConstraintSchema,
    UniqueConstraintName,
    MatchOption,
    UpdateRule,
    DeleteRule,
}
