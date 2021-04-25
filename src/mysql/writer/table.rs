use sea_query::{Alias, Iden, Table, TableStatement};
use crate::mysql::def::TableDef;

impl TableDef {
    pub fn write(&self) -> TableStatement {
        let mut table = Table::create();
        table.table(Alias::new(self.info.name.as_ref()));
        for col in self.columns.iter() {
            table.col(col.write());
        }
        table.engine(self.info.engine.to_string().as_str());
        table.character_set(self.info.char_set.to_string().as_str());
        table.collate(self.info.collation.to_string().as_str());
        for idx in self.indexes.iter() {
            table.index(idx.write());
        }
        // for key in self.foreign_keys.iter() {
        //     table.foreign_key(key.write());
        // }
        TableStatement::Create(table)
    }
}

#[cfg(test)]
mod tests {
    use sea_query::{MysqlQueryBuilder};
    use crate::mysql::def::*;

    #[test]
    fn test_1() {
        assert_eq!(
            TableDef {
                info: TableInfo {
                    name: "actor".to_owned(),
                    engine: StorageEngine::InnoDb,
                    auto_increment: None,
                    char_set: CharSet::Utf8Mb4,
                    collation: Collation::Utf8Mb40900AiCi,
                    comment: "".to_owned(),
                },
                columns: vec![
                    ColumnInfo {
                        name: "actor_id".to_owned(),
                        col_type: ColumnType::SmallInt(
                            NumericAttr {
                                maximum: None,
                                decimal: None,
                                unsigned: Some(
                                    true,
                                ),
                                zero_fill: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::Primary,
                        default: None,
                        extra: ColumnExtra {
                            auto_increment: true,
                            on_update_current_timestamp: false,
                            generated: false,
                            default_generated: false,
                        },
                        expression: None,
                        comment: "Actor ID".to_owned(),
                    },
                    ColumnInfo {
                        name: "first_name".to_owned(),
                        col_type: ColumnType::Varchar(
                            StringAttr {
                                length: Some(
                                    45,
                                ),
                                charset: None,
                                collation: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::NotKey,
                        default: None,
                        extra: ColumnExtra {
                            auto_increment: false,
                            on_update_current_timestamp: false,
                            generated: false,
                            default_generated: false,
                        },
                        expression: None,
                        comment: "".to_owned(),
                    },
                    ColumnInfo {
                        name: "last_name".to_owned(),
                        col_type: ColumnType::Varchar(
                            StringAttr {
                                length: Some(
                                    45,
                                ),
                                charset: None,
                                collation: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::Multiple,
                        default: None,
                        extra: ColumnExtra {
                            auto_increment: false,
                            on_update_current_timestamp: false,
                            generated: false,
                            default_generated: false,
                        },
                        expression: None,
                        comment: "".to_owned(),
                    },
                    ColumnInfo {
                        name: "last_update".to_owned(),
                        col_type: ColumnType::Timestamp(
                            TimeAttr {
                                fractional: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::NotKey,
                        default: Some(
                            ColumnDefault {
                                expr: "CURRENT_TIMESTAMP".to_owned(),
                            },
                        ),
                        extra: ColumnExtra {
                            auto_increment: false,
                            on_update_current_timestamp: true,
                            generated: false,
                            default_generated: true,
                        },
                        expression: None,
                        comment: "".to_owned(),
                    },
                ],
                indexes: vec![],
                foreign_keys: vec![],
            }.write().to_string(MysqlQueryBuilder),
            vec![
                "CREATE TABLE `actor` (",
                    "`actor_id` SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'Actor ID',",
                    "`first_name` VARCHAR(45) NOT NULL,",
                    "`last_name` VARCHAR(45) NOT NULL,",
                    "`last_update` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                ")",
                "ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci",
            ].join(" ")
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            TableDef {
                info: TableInfo {
                    name: "film_actor".to_owned(),
                    engine: StorageEngine::InnoDb,
                    auto_increment: None,
                    char_set: CharSet::Utf8Mb4,
                    collation: Collation::Utf8Mb40900AiCi,
                    comment: "".to_owned(),
                },
                columns: vec![
                    ColumnInfo {
                        name: "actor_id".to_owned(),
                        col_type: ColumnType::SmallInt(
                            NumericAttr {
                                maximum: None,
                                decimal: None,
                                unsigned: Some(
                                    true,
                                ),
                                zero_fill: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::Primary,
                        default: None,
                        extra: ColumnExtra {
                            auto_increment: false,
                            on_update_current_timestamp: false,
                            generated: false,
                            default_generated: false,
                        },
                        expression: None,
                        comment: "".to_owned(),
                    },
                    ColumnInfo {
                        name: "film_id".to_owned(),
                        col_type: ColumnType::SmallInt(
                            NumericAttr {
                                maximum: None,
                                decimal: None,
                                unsigned: Some(
                                    true,
                                ),
                                zero_fill: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::Primary,
                        default: None,
                        extra: ColumnExtra {
                            auto_increment: false,
                            on_update_current_timestamp: false,
                            generated: false,
                            default_generated: false,
                        },
                        expression: None,
                        comment: "".to_owned(),
                    },
                    ColumnInfo {
                        name: "last_update".to_owned(),
                        col_type: ColumnType::Timestamp(
                            TimeAttr {
                                fractional: None,
                            },
                        ),
                        null: false,
                        key: ColumnKey::NotKey,
                        default: Some(
                            ColumnDefault {
                                expr: "CURRENT_TIMESTAMP".to_owned(),
                            },
                        ),
                        extra: ColumnExtra {
                            auto_increment: false,
                            on_update_current_timestamp: true,
                            generated: false,
                            default_generated: true,
                        },
                        expression: None,
                        comment: "".to_owned(),
                    },
                ],
                indexes: vec![
                    IndexInfo {
                        unique: true,
                        name: "PRIMARY".to_owned(),
                        parts: vec![
                            IndexPart {
                                column: "actor_id".to_owned(),
                                order: IndexOrder::Ascending,
                                sub_part: None,
                            },
                            IndexPart {
                                column: "film_id".to_owned(),
                                order: IndexOrder::Ascending,
                                sub_part: None,
                            },
                        ],
                        nullable: false,
                        idx_type: IndexType::BTree,
                        comment: "".to_owned(),
                        functional: false,
                    },
                    IndexInfo {
                        unique: false,
                        name: "idx_fk_film_id".to_owned(),
                        parts: vec![
                            IndexPart {
                                column: "film_id".to_owned(),
                                order: IndexOrder::Ascending,
                                sub_part: None,
                            },
                        ],
                        nullable: false,
                        idx_type: IndexType::BTree,
                        comment: "".to_owned(),
                        functional: false,
                    },
                ],
                foreign_keys: vec![],
            }.write().to_string(MysqlQueryBuilder),
            vec![
                "CREATE TABLE `film_actor` (",
                    "`actor_id` SMALLINT UNSIGNED NOT NULL,",
                    "`film_id` SMALLINT UNSIGNED NOT NULL,",
                    "`last_update` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,",
                    "PRIMARY KEY (`actor_id`, `film_id`),",
                    "KEY `idx_fk_film_id` (`film_id`)",
                ")",
                "ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci",
            ].join(" ")
        );
    }
}