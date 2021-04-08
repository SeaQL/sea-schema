<div align="center">

  <img src="docs/SeaQL logo dual.png" width="320"/>

  <h1>SeaSchema</h1>

  <p>
    <strong>Database schema definition, discovery and conversion</strong>
  </p>

  [![crate](https://img.shields.io/crates/v/sea-query.svg)](https://crates.io/crates/sea-query)
  [![docs](https://docs.rs/sea-query/badge.svg)](https://docs.rs/sea-query)
  [![build status](https://github.com/SeaQL/sea-query/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-query/actions/workflows/rust.yml)

  <sub>Built with ❤️ by 🌊🦀🐚</sub>

</div>

## Introduction

SeaSchema is a library to help you manage database schema. It provides data structures for 
representing schema definition and query helpers to discover such schema from INFORMATION_SCHEMA.

For now, MySQL support is almost done and Postgres support is in progress. At the end, we'd want to 
support schema conversion between MySQL and Postgres.

## Example

Take the MySQL Sakila Sample Database as example, given the following table:

```SQL
CREATE TABLE actor (
  actor_id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT,
  first_name VARCHAR(45) NOT NULL,
  last_name VARCHAR(45) NOT NULL,
  last_update TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY  (actor_id),
  KEY idx_actor_last_name (last_name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

The discovered schema by querying INFORMATION_SCHEMA results in:

```rust
TableDef {
    info: TableInfo {
        name: "actor",
        engine: InnoDb,
        auto_increment: None,
        collation: Utf8Mb40900AiCi,
        comment: "",
    },
    columns: [
        ColumnInfo {
            name: "actor_id",
            col_type: SmallInt(
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
            key: Primary,
            default: None,
            extra: ColumnExtra {
                auto_increment: true,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: false,
            },
            expression: None,
            comment: "",
        },
        ColumnInfo {
            name: "first_name",
            col_type: Varchar(
                StringAttr {
                    length: Some(
                        45,
                    ),
                    charset_name: None,
                    collation_name: None,
                },
            ),
            null: false,
            key: NotKey,
            default: None,
            extra: ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: false,
            },
            expression: None,
            comment: "",
        },
        ColumnInfo {
            name: "last_name",
            col_type: Varchar(
                StringAttr {
                    length: Some(
                        45,
                    ),
                    charset_name: None,
                    collation_name: None,
                },
            ),
            null: false,
            key: Multiple,
            default: None,
            extra: ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: false,
            },
            expression: None,
            comment: "",
        },
        ColumnInfo {
            name: "last_update",
            col_type: Timestamp(
                TimeAttr {
                    fractional: None,
                },
            ),
            null: false,
            key: NotKey,
            default: Some(
                ColumnDefault {
                    expr: "CURRENT_TIMESTAMP",
                },
            ),
            extra: ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: true,
                generated: false,
                default_generated: true,
            },
            expression: None,
            comment: "",
        },
    ],
    indexes: [
        IndexInfo {
            unique: false,
            name: "idx_actor_last_name",
            columns: [
                "last_name",
            ],
            order: Ascending,
            sub_part: None,
            nullable: false,
            idx_type: BTree,
            comment: "",
            functional: false,
        },
        IndexInfo {
            unique: true,
            name: "PRIMARY",
            columns: [
                "actor_id",
            ],
            order: Ascending,
            sub_part: None,
            nullable: false,
            idx_type: BTree,
            comment: "",
            functional: false,
        },
    ],
    foreign_keys: [],
},
```