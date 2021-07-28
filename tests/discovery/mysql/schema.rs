Schema {
    system: SystemInfo {
        version: 80023,
        system: "0ubuntu0.20.04.1",
        suffix: [],
    },
    tables: [
        TableDef {
            info: TableInfo {
                name: "actor",
                engine: InnoDb,
                auto_increment: Some(
                    200,
                ),
                char_set: Utf8Mb4,
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
                            charset: None,
                            collation: None,
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
                            charset: None,
                            collation: None,
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
                    parts: [
                        IndexPart {
                            column: "last_name",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "actor_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [],
        },
        TableDef {
            info: TableInfo {
                name: "address",
                engine: InnoDb,
                auto_increment: Some(
                    605,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "address_id",
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
                    name: "address",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "address2",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
                    name: "district",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                20,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "city_id",
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
                    name: "postal_code",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                10,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
                    name: "phone",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                20,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "location",
                    col_type: Geometry(
                        GeometryAttr {
                            srid: None,
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
                    name: "idx_fk_city_id",
                    parts: [
                        IndexPart {
                            column: "city_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_location",
                    parts: [
                        IndexPart {
                            column: "location",
                            order: Ascending,
                            sub_part: Some(
                                32,
                            ),
                        },
                    ],
                    nullable: false,
                    idx_type: Spatial,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "address_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_address_city",
                    columns: [
                        "city_id",
                    ],
                    referenced_table: "city",
                    referenced_columns: [
                        "city_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "category",
                engine: InnoDb,
                auto_increment: Some(
                    16,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "category_id",
                    col_type: TinyInt(
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
                    name: "name",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                25,
                            ),
                            charset: None,
                            collation: None,
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
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "category_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [],
        },
        TableDef {
            info: TableInfo {
                name: "city",
                engine: InnoDb,
                auto_increment: Some(
                    600,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "city_id",
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
                    name: "city",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "country_id",
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
                    name: "idx_fk_country_id",
                    parts: [
                        IndexPart {
                            column: "country_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "city_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_city_country",
                    columns: [
                        "country_id",
                    ],
                    referenced_table: "country",
                    referenced_columns: [
                        "country_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "country",
                engine: InnoDb,
                auto_increment: Some(
                    109,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "country_id",
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
                    name: "country",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                            charset: None,
                            collation: None,
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
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "country_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [],
        },
        TableDef {
            info: TableInfo {
                name: "customer",
                engine: InnoDb,
                auto_increment: Some(
                    599,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "customer_id",
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
                    name: "store_id",
                    col_type: TinyInt(
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
                    name: "first_name",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                            charset: None,
                            collation: None,
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
                            charset: None,
                            collation: None,
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
                    name: "email",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
                    name: "address_id",
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
                    name: "active",
                    col_type: TinyInt(
                        NumericAttr {
                            maximum: Some(
                                1,
                            ),
                            decimal: None,
                            unsigned: None,
                            zero_fill: None,
                        },
                    ),
                    null: false,
                    key: NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "1",
                        },
                    ),
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
                    name: "create_date",
                    col_type: DateTime(
                        TimeAttr {
                            fractional: None,
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
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            fractional: None,
                        },
                    ),
                    null: true,
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
                    name: "idx_fk_address_id",
                    parts: [
                        IndexPart {
                            column: "address_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_store_id",
                    parts: [
                        IndexPart {
                            column: "store_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_last_name",
                    parts: [
                        IndexPart {
                            column: "last_name",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "customer_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_customer_address",
                    columns: [
                        "address_id",
                    ],
                    referenced_table: "address",
                    referenced_columns: [
                        "address_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_customer_store",
                    columns: [
                        "store_id",
                    ],
                    referenced_table: "store",
                    referenced_columns: [
                        "store_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "film",
                engine: InnoDb,
                auto_increment: Some(
                    1000,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "film_id",
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
                    name: "title",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                128,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "description",
                    col_type: Text(
                        StringAttr {
                            length: None,
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
                    name: "release_year",
                    col_type: Year,
                    null: true,
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
                    name: "language_id",
                    col_type: TinyInt(
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
                    name: "original_language_id",
                    col_type: TinyInt(
                        NumericAttr {
                            maximum: None,
                            decimal: None,
                            unsigned: Some(
                                true,
                            ),
                            zero_fill: None,
                        },
                    ),
                    null: true,
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
                    name: "rental_duration",
                    col_type: TinyInt(
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
                    key: NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "3",
                        },
                    ),
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
                    name: "rental_rate",
                    col_type: Decimal(
                        NumericAttr {
                            maximum: Some(
                                4,
                            ),
                            decimal: Some(
                                2,
                            ),
                            unsigned: None,
                            zero_fill: None,
                        },
                    ),
                    null: false,
                    key: NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "4.99",
                        },
                    ),
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
                    name: "length",
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
                    null: true,
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
                    name: "replacement_cost",
                    col_type: Decimal(
                        NumericAttr {
                            maximum: Some(
                                5,
                            ),
                            decimal: Some(
                                2,
                            ),
                            unsigned: None,
                            zero_fill: None,
                        },
                    ),
                    null: false,
                    key: NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "19.99",
                        },
                    ),
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
                    name: "rating",
                    col_type: Enum(
                        EnumDef {
                            values: [
                                "G",
                                "PG",
                                "PG-13",
                                "R",
                                "NC-17",
                            ],
                            attr: StringAttr {
                                length: None,
                                charset: None,
                                collation: None,
                            },
                        },
                    ),
                    null: true,
                    key: NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "G",
                        },
                    ),
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
                    name: "special_features",
                    col_type: Set(
                        SetDef {
                            members: [
                                "Trailers",
                                "Commentaries",
                                "Deleted Scenes",
                                "Behind the Scenes",
                            ],
                            attr: StringAttr {
                                length: None,
                                charset: None,
                                collation: None,
                            },
                        },
                    ),
                    null: true,
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
                    name: "idx_fk_language_id",
                    parts: [
                        IndexPart {
                            column: "language_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_original_language_id",
                    parts: [
                        IndexPart {
                            column: "original_language_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: true,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_title",
                    parts: [
                        IndexPart {
                            column: "title",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_film_language",
                    columns: [
                        "language_id",
                    ],
                    referenced_table: "language",
                    referenced_columns: [
                        "language_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_film_language_original",
                    columns: [
                        "original_language_id",
                    ],
                    referenced_table: "language",
                    referenced_columns: [
                        "language_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "film_actor",
                engine: InnoDb,
                auto_increment: None,
                char_set: Utf8Mb4,
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
                        auto_increment: false,
                        on_update_current_timestamp: false,
                        generated: false,
                        default_generated: false,
                    },
                    expression: None,
                    comment: "",
                },
                ColumnInfo {
                    name: "film_id",
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
                    name: "idx_fk_film_id",
                    parts: [
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "actor_id",
                            order: Ascending,
                            sub_part: None,
                        },
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_film_actor_actor",
                    columns: [
                        "actor_id",
                    ],
                    referenced_table: "actor",
                    referenced_columns: [
                        "actor_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_film_actor_film",
                    columns: [
                        "film_id",
                    ],
                    referenced_table: "film",
                    referenced_columns: [
                        "film_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "film_category",
                engine: InnoDb,
                auto_increment: None,
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "film_id",
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
                        auto_increment: false,
                        on_update_current_timestamp: false,
                        generated: false,
                        default_generated: false,
                    },
                    expression: None,
                    comment: "",
                },
                ColumnInfo {
                    name: "category_id",
                    col_type: TinyInt(
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
                    name: "fk_film_category_category",
                    parts: [
                        IndexPart {
                            column: "category_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                        IndexPart {
                            column: "category_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_film_category_category",
                    columns: [
                        "category_id",
                    ],
                    referenced_table: "category",
                    referenced_columns: [
                        "category_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_film_category_film",
                    columns: [
                        "film_id",
                    ],
                    referenced_table: "film",
                    referenced_columns: [
                        "film_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "film_text",
                engine: InnoDb,
                auto_increment: None,
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt(
                        NumericAttr {
                            maximum: None,
                            decimal: None,
                            unsigned: None,
                            zero_fill: None,
                        },
                    ),
                    null: false,
                    key: Primary,
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
                    name: "title",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                255,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "description",
                    col_type: Text(
                        StringAttr {
                            length: None,
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
            ],
            indexes: [
                IndexInfo {
                    unique: false,
                    name: "idx_title_description",
                    parts: [
                        IndexPart {
                            column: "title",
                            order: Unordered,
                            sub_part: None,
                        },
                        IndexPart {
                            column: "description",
                            order: Unordered,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: FullText,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [],
        },
        TableDef {
            info: TableInfo {
                name: "inventory",
                engine: InnoDb,
                auto_increment: Some(
                    4581,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "inventory_id",
                    col_type: MediumInt(
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
                    name: "film_id",
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
                    name: "store_id",
                    col_type: TinyInt(
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
                    name: "idx_fk_film_id",
                    parts: [
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_store_id_film_id",
                    parts: [
                        IndexPart {
                            column: "store_id",
                            order: Ascending,
                            sub_part: None,
                        },
                        IndexPart {
                            column: "film_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "inventory_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_inventory_film",
                    columns: [
                        "film_id",
                    ],
                    referenced_table: "film",
                    referenced_columns: [
                        "film_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_inventory_store",
                    columns: [
                        "store_id",
                    ],
                    referenced_table: "store",
                    referenced_columns: [
                        "store_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "language",
                engine: InnoDb,
                auto_increment: Some(
                    6,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "language_id",
                    col_type: TinyInt(
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
                    name: "name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                20,
                            ),
                            charset: None,
                            collation: None,
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
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "language_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment",
                engine: InnoDb,
                auto_increment: Some(
                    16049,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
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
                    name: "customer_id",
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
                    name: "staff_id",
                    col_type: TinyInt(
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
                    name: "rental_id",
                    col_type: Int(
                        NumericAttr {
                            maximum: None,
                            decimal: None,
                            unsigned: None,
                            zero_fill: None,
                        },
                    ),
                    null: true,
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
                    name: "amount",
                    col_type: Decimal(
                        NumericAttr {
                            maximum: Some(
                                5,
                            ),
                            decimal: Some(
                                2,
                            ),
                            unsigned: None,
                            zero_fill: None,
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
                    name: "payment_date",
                    col_type: DateTime(
                        TimeAttr {
                            fractional: None,
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
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            fractional: None,
                        },
                    ),
                    null: true,
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
                    name: "fk_payment_rental",
                    parts: [
                        IndexPart {
                            column: "rental_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: true,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_customer_id",
                    parts: [
                        IndexPart {
                            column: "customer_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_staff_id",
                    parts: [
                        IndexPart {
                            column: "staff_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "payment_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_payment_customer",
                    columns: [
                        "customer_id",
                    ],
                    referenced_table: "customer",
                    referenced_columns: [
                        "customer_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_payment_rental",
                    columns: [
                        "rental_id",
                    ],
                    referenced_table: "rental",
                    referenced_columns: [
                        "rental_id",
                    ],
                    on_update: Cascade,
                    on_delete: SetNull,
                },
                ForeignKeyInfo {
                    name: "fk_payment_staff",
                    columns: [
                        "staff_id",
                    ],
                    referenced_table: "staff",
                    referenced_columns: [
                        "staff_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "rental",
                engine: InnoDb,
                auto_increment: Some(
                    16049,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "rental_id",
                    col_type: Int(
                        NumericAttr {
                            maximum: None,
                            decimal: None,
                            unsigned: None,
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
                    name: "rental_date",
                    col_type: DateTime(
                        TimeAttr {
                            fractional: None,
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
                    name: "inventory_id",
                    col_type: MediumInt(
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
                    name: "customer_id",
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
                    name: "return_date",
                    col_type: DateTime(
                        TimeAttr {
                            fractional: None,
                        },
                    ),
                    null: true,
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
                    name: "staff_id",
                    col_type: TinyInt(
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
                    name: "idx_fk_customer_id",
                    parts: [
                        IndexPart {
                            column: "customer_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_inventory_id",
                    parts: [
                        IndexPart {
                            column: "inventory_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_staff_id",
                    parts: [
                        IndexPart {
                            column: "staff_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "rental_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "rental_date",
                    parts: [
                        IndexPart {
                            column: "rental_date",
                            order: Ascending,
                            sub_part: None,
                        },
                        IndexPart {
                            column: "inventory_id",
                            order: Ascending,
                            sub_part: None,
                        },
                        IndexPart {
                            column: "customer_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_rental_customer",
                    columns: [
                        "customer_id",
                    ],
                    referenced_table: "customer",
                    referenced_columns: [
                        "customer_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_rental_inventory",
                    columns: [
                        "inventory_id",
                    ],
                    referenced_table: "inventory",
                    referenced_columns: [
                        "inventory_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_rental_staff",
                    columns: [
                        "staff_id",
                    ],
                    referenced_table: "staff",
                    referenced_columns: [
                        "staff_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "staff",
                engine: InnoDb,
                auto_increment: Some(
                    2,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "staff_id",
                    col_type: TinyInt(
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
                            charset: None,
                            collation: None,
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
                            charset: None,
                            collation: None,
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
                    name: "address_id",
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
                    name: "picture",
                    col_type: Blob(
                        BlobAttr {
                            length: None,
                        },
                    ),
                    null: true,
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
                    name: "email",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
                    name: "store_id",
                    col_type: TinyInt(
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
                    name: "active",
                    col_type: TinyInt(
                        NumericAttr {
                            maximum: Some(
                                1,
                            ),
                            decimal: None,
                            unsigned: None,
                            zero_fill: None,
                        },
                    ),
                    null: false,
                    key: NotKey,
                    default: Some(
                        ColumnDefault {
                            expr: "1",
                        },
                    ),
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
                    name: "username",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                16,
                            ),
                            charset: None,
                            collation: None,
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
                    name: "password",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                40,
                            ),
                            charset: None,
                            collation: None,
                        },
                    ),
                    null: true,
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
                    name: "idx_fk_address_id",
                    parts: [
                        IndexPart {
                            column: "address_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: false,
                    name: "idx_fk_store_id",
                    parts: [
                        IndexPart {
                            column: "store_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "staff_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_staff_address",
                    columns: [
                        "address_id",
                    ],
                    referenced_table: "address",
                    referenced_columns: [
                        "address_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_staff_store",
                    columns: [
                        "store_id",
                    ],
                    referenced_table: "store",
                    referenced_columns: [
                        "store_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
        TableDef {
            info: TableInfo {
                name: "store",
                engine: InnoDb,
                auto_increment: Some(
                    2,
                ),
                char_set: Utf8Mb4,
                collation: Utf8Mb40900AiCi,
                comment: "",
            },
            columns: [
                ColumnInfo {
                    name: "store_id",
                    col_type: TinyInt(
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
                    name: "manager_staff_id",
                    col_type: TinyInt(
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
                    key: Unique,
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
                    name: "address_id",
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
                    name: "idx_fk_address_id",
                    parts: [
                        IndexPart {
                            column: "address_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "idx_unique_manager",
                    parts: [
                        IndexPart {
                            column: "manager_staff_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
                IndexInfo {
                    unique: true,
                    name: "PRIMARY",
                    parts: [
                        IndexPart {
                            column: "store_id",
                            order: Ascending,
                            sub_part: None,
                        },
                    ],
                    nullable: false,
                    idx_type: BTree,
                    comment: "",
                    functional: false,
                },
            ],
            foreign_keys: [
                ForeignKeyInfo {
                    name: "fk_store_address",
                    columns: [
                        "address_id",
                    ],
                    referenced_table: "address",
                    referenced_columns: [
                        "address_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
                ForeignKeyInfo {
                    name: "fk_store_staff",
                    columns: [
                        "manager_staff_id",
                    ],
                    referenced_table: "staff",
                    referenced_columns: [
                        "staff_id",
                    ],
                    on_update: Cascade,
                    on_delete: Restrict,
                },
            ],
        },
    ],
}
