#[derive(Debug, sea_query::Iden)]
/// Refs:
/// - https://dev.mysql.com/doc/refman/8.0/en/information-schema-collation-character-set-applicability-table.html
/// - https://mariadb.com/docs/server/reference/system-tables/information-schema/information-schema-tables/information-schema-collation_character_set_applicability-table
pub enum CharacterSetFields {
    CharacterSetName,
    CollationName,
    // Used by mariadb
    #[iden = "FULL_COLLATION_NAME"]
    FullCollationName,
}
