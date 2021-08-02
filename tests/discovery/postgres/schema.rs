Schema {
    schema: "public",
    tables: [
        TableDef {
            info: TableInfo {
                name: "film",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "film_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('film_film_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "title",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                255,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "description",
                    col_type: Text,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "release_year",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "language_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "original_language_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_duration",
                    col_type: SmallInt,
                    default: Some(
                        ColumnExpression(
                            "3",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_rate",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                4,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "4.99",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "length",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "replacement_cost",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "19.99",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rating",
                    col_type: Unknown(
                        "USER",
                    ),
                    default: Some(
                        ColumnExpression(
                            "'G'::mpaa_rating",
                        ),
                    ),
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "special_features",
                    col_type: Array,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "fulltext",
                    col_type: TsVector,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "replacement_cost IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "fulltext IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "title IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_duration IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_rate IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "film_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "language_id",
                    ],
                    table: "language",
                    foreign_columns: [
                        "language_id",
                    ],
                },
                References {
                    columns: [
                        "original_language_id",
                    ],
                    table: "language",
                    foreign_columns: [
                        "language_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "address",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "address_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('address_address_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "address",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "address2",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "district",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                20,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "city_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "postal_code",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                10,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "phone",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                20,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "address IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "district IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "city_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "phone IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "address_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "city_id",
                    ],
                    table: "city",
                    foreign_columns: [
                        "city_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "category",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "category_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('category_category_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                25,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "category_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "category_id",
                    ],
                ),
            ],
            reference_constraints: [],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "city",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "city_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('city_city_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "city",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "country_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "city_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "city IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "country_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "city_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "country_id",
                    ],
                    table: "country",
                    foreign_columns: [
                        "country_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "country",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "country_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('country_country_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "country",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "country_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "country IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "country_id",
                    ],
                ),
            ],
            reference_constraints: [],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "customer",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "customer_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('customer_customer_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "email",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "activebool",
                    col_type: Boolean,
                    default: Some(
                        ColumnExpression(
                            "true",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "create_date",
                    col_type: Date,
                    default: Some(
                        ColumnExpression(
                            "('now'::text)::date",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "active",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
                },
            ],
            check_constraints: [
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "activebool IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "create_date IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "customer_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "address_id",
                    ],
                    table: "address",
                    foreign_columns: [
                        "address_id",
                    ],
                },
                References {
                    columns: [
                        "store_id",
                    ],
                    table: "store",
                    foreign_columns: [
                        "store_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "film_actor",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "actor_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "actor_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "actor_id",
                        "film_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "actor_id",
                    ],
                    table: "actor",
                    foreign_columns: [
                        "actor_id",
                    ],
                },
                References {
                    columns: [
                        "film_id",
                    ],
                    table: "film",
                    foreign_columns: [
                        "film_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "film_category",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "category_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "category_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "film_id",
                        "category_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "category_id",
                    ],
                    table: "category",
                    foreign_columns: [
                        "category_id",
                    ],
                },
                References {
                    columns: [
                        "film_id",
                    ],
                    table: "film",
                    foreign_columns: [
                        "film_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "inventory",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "inventory_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('inventory_inventory_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "inventory_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "inventory_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "film_id",
                    ],
                    table: "film",
                    foreign_columns: [
                        "film_id",
                    ],
                },
                References {
                    columns: [
                        "store_id",
                    ],
                    table: "store",
                    foreign_columns: [
                        "store_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "language",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "language_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('language_language_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                20,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "language_id",
                    ],
                ),
            ],
            reference_constraints: [],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "rental",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('rental_rental_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "inventory_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "return_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "inventory_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "rental_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "inventory_id",
                    ],
                    table: "inventory",
                    foreign_columns: [
                        "inventory_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "staff",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "staff_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('staff_staff_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "email",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "active",
                    col_type: Boolean,
                    default: Some(
                        ColumnExpression(
                            "true",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "username",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                16,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "password",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                40,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "picture",
                    col_type: Bytea,
                    default: None,
                    generated: None,
                    not_null: None,
                },
            ],
            check_constraints: [
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "active IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "username IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "staff_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "address_id",
                    ],
                    table: "address",
                    foreign_columns: [
                        "address_id",
                    ],
                },
                References {
                    columns: [
                        "store_id",
                    ],
                    table: "store",
                    foreign_columns: [
                        "store_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "store",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "store_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('store_store_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "manager_staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "manager_staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "store_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "address_id",
                    ],
                    table: "address",
                    foreign_columns: [
                        "address_id",
                    ],
                },
                References {
                    columns: [
                        "manager_staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "payment_id",
                    ],
                ),
            ],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment_p2007_01",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "(((payment_date >= '2007-01-01 00:00:00'::timestamp without time zone) AND (payment_date < '2007-02-01 00:00:00'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment_p2007_02",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "(((payment_date >= '2007-02-01 00:00:00'::timestamp without time zone) AND (payment_date < '2007-03-01 00:00:00'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment_p2007_03",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "(((payment_date >= '2007-03-01 00:00:00'::timestamp without time zone) AND (payment_date < '2007-04-01 00:00:00'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment_p2007_04",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "(((payment_date >= '2007-04-01 00:00:00'::timestamp without time zone) AND (payment_date < '2007-05-01 00:00:00'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment_p2007_05",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "(((payment_date >= '2007-05-01 00:00:00'::timestamp without time zone) AND (payment_date < '2007-06-01 00:00:00'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "payment_p2007_06",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('payment_payment_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "amount",
                    col_type: Numeric(
                        ArbitraryPrecisionNumericAttr {
                            precision: Some(
                                5,
                            ),
                            scale: Some(
                                2,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
            ],
            check_constraints: [
                Check {
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "(((payment_date >= '2007-06-01 00:00:00'::timestamp without time zone) AND (payment_date < '2007-07-01 00:00:00'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                },
                References {
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                },
                References {
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                },
            ],
            exclusion_constraints: [],
        },
        TableDef {
            info: TableInfo {
                name: "actor",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "actor_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval('actor_actor_id_seq'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Char(
                        StringAttr {
                            length: Some(
                                45,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Timestamp(
                        TimeAttr {
                            precision: Some(
                                6,
                            ),
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "interval",
                    col_type: Interval(
                        IntervalAttr {
                            field: Some(
                                "DAY TO SECOND(2)",
                            ),
                            precision: None,
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "bit",
                    col_type: Bit(
                        BitAttr {
                            length: Some(
                                20,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "bit_varying",
                    col_type: Bit(
                        BitAttr {
                            length: Some(
                                10,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "ts_vector",
                    col_type: TsVector,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "ts_query",
                    col_type: TsQuery,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "uuid",
                    col_type: Uuid,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "xml",
                    col_type: Xml,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "json",
                    col_type: Json,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "array_int",
                    col_type: Array,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "array_int_int",
                    col_type: Array,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "pg_lsn",
                    col_type: PgLsn,
                    default: None,
                    generated: None,
                    not_null: None,
                },
            ],
            check_constraints: [
                Check {
                    expr: "actor_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [
                Unique(
                    [
                        "actor_id",
                    ],
                ),
            ],
            primary_key_constraints: [
                PrimaryKey(
                    [
                        "actor_id",
                    ],
                ),
            ],
            reference_constraints: [],
            exclusion_constraints: [],
        },
    ],
}
