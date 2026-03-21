Schema {
    schema: "public",
    tables: [
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Varchar(
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
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "actor_actor_id_not_null",
                    expr: "actor_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "actor_first_name_not_null",
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "actor_last_name_not_null",
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "actor_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "actor_pkey",
                    columns: [
                        "actor_id",
                    ],
                },
            ],
            reference_constraints: [],
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "address",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "address2",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "district",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "city_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "postal_code",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                10,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "phone",
                    col_type: Varchar(
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
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "address_address_id_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "address_address_not_null",
                    expr: "address IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "address_city_id_not_null",
                    expr: "city_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "address_district_not_null",
                    expr: "district IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "address_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "address_phone_not_null",
                    expr: "phone IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "address_pkey",
                    columns: [
                        "address_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "address_city_id_fkey",
                    columns: [
                        "city_id",
                    ],
                    table: "city",
                    foreign_columns: [
                        "city_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "name",
                    col_type: Varchar(
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
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "category_category_id_not_null",
                    expr: "category_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "category_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "category_name_not_null",
                    expr: "name IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "category_pkey",
                    columns: [
                        "category_id",
                    ],
                },
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "city",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "country_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "city_city_id_not_null",
                    expr: "city_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "city_city_not_null",
                    expr: "city IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "city_country_id_not_null",
                    expr: "country_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "city_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "city_pkey",
                    columns: [
                        "city_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "city_country_id_fkey",
                    columns: [
                        "country_id",
                    ],
                    table: "country",
                    foreign_columns: [
                        "country_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
            ],
            exclusion_constraints: [],
        },
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "title",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "description",
                    col_type: Text,
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "release_year",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "language_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "original_language_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
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
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "length",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
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
                    is_identity: false,
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
                            typename: "mpaa_rating",
                            schema: "",
                        },
                    ),
                    default: Some(
                        ColumnExpression(
                            "'G'::mpaa_rating",
                        ),
                    ),
                    generated: None,
                    not_null: None,
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "special_features",
                    col_type: Array(
                        ArrayDef {
                            col_type: Some(
                                Text,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "fulltext",
                    col_type: TsVector,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "film_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_fulltext_not_null",
                    expr: "fulltext IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_fulltext_not_null",
                    expr: "fulltext IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_fulltext_not_null",
                    expr: "fulltext IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_fulltext_not_null",
                    expr: "fulltext IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_language_id_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_language_id_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_language_id_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_language_id_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_duration_not_null",
                    expr: "rental_duration IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_duration_not_null",
                    expr: "rental_duration IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_duration_not_null",
                    expr: "rental_duration IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_duration_not_null",
                    expr: "rental_duration IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_rate_not_null",
                    expr: "rental_rate IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_rate_not_null",
                    expr: "rental_rate IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_rate_not_null",
                    expr: "rental_rate IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_rental_rate_not_null",
                    expr: "rental_rate IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_replacement_cost_not_null",
                    expr: "replacement_cost IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_replacement_cost_not_null",
                    expr: "replacement_cost IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_replacement_cost_not_null",
                    expr: "replacement_cost IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_replacement_cost_not_null",
                    expr: "replacement_cost IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_title_not_null",
                    expr: "title IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_title_not_null",
                    expr: "title IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_title_not_null",
                    expr: "title IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_title_not_null",
                    expr: "title IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "film_pkey",
                    columns: [
                        "film_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "film_language_id_fkey",
                    columns: [
                        "language_id",
                    ],
                    table: "language",
                    foreign_columns: [
                        "language_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "film_original_language_id_fkey",
                    columns: [
                        "original_language_id",
                    ],
                    table: "language",
                    foreign_columns: [
                        "language_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "country",
                    col_type: Varchar(
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
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "country_country_id_not_null",
                    expr: "country_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "country_country_not_null",
                    expr: "country IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "country_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "country_pkey",
                    columns: [
                        "country_id",
                    ],
                },
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "email",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
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
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "active",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "customer_activebool_not_null",
                    expr: "activebool IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "customer_address_id_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "customer_create_date_not_null",
                    expr: "create_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "customer_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "customer_first_name_not_null",
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "customer_last_name_not_null",
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "customer_store_id_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "customer_pkey",
                    columns: [
                        "customer_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "customer_address_id_fkey",
                    columns: [
                        "address_id",
                    ],
                    table: "address",
                    foreign_columns: [
                        "address_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "customer_store_id_fkey",
                    columns: [
                        "store_id",
                    ],
                    table: "store",
                    foreign_columns: [
                        "store_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "film_actor_actor_id_not_null",
                    expr: "actor_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_actor_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_actor_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "film_actor_pkey",
                    columns: [
                        "actor_id",
                        "film_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "film_actor_actor_id_fkey",
                    columns: [
                        "actor_id",
                    ],
                    table: "actor",
                    foreign_columns: [
                        "actor_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "category_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "film_category_category_id_not_null",
                    expr: "category_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_category_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "film_category_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "film_category_pkey",
                    columns: [
                        "film_id",
                        "category_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "film_category_category_id_fkey",
                    columns: [
                        "category_id",
                    ],
                    table: "category",
                    foreign_columns: [
                        "category_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "inventory_film_id_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "inventory_inventory_id_not_null",
                    expr: "inventory_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "inventory_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "inventory_store_id_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "inventory_pkey",
                    columns: [
                        "inventory_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "inventory_store_id_fkey",
                    columns: [
                        "store_id",
                    ],
                    table: "store",
                    foreign_columns: [
                        "store_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
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
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "language_language_id_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "language_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "language_name_not_null",
                    expr: "name IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "language_pkey",
                    columns: [
                        "language_id",
                    ],
                },
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
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "inventory_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "rental_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "rental_inventory_id_not_null",
                    expr: "inventory_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "rental_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "rental_rental_date_not_null",
                    expr: "rental_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "rental_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "rental_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [
                Unique {
                    name: "idx_unq_rental_rental_date_inventory_id_customer_id",
                    columns: [
                        "rental_date",
                        "inventory_id",
                        "customer_id",
                    ],
                    is_partial: false,
                },
            ],
            primary_key_constraints: [
                PrimaryKey {
                    name: "rental_pkey",
                    columns: [
                        "rental_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "rental_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "rental_inventory_id_fkey",
                    columns: [
                        "inventory_id",
                    ],
                    table: "inventory",
                    foreign_columns: [
                        "inventory_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "rental_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "email",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                50,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "username",
                    col_type: Varchar(
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "password",
                    col_type: Varchar(
                        StringAttr {
                            length: Some(
                                40,
                            ),
                        },
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "picture",
                    col_type: Bytea,
                    default: None,
                    generated: None,
                    not_null: None,
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "staff_active_not_null",
                    expr: "active IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_address_id_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_first_name_not_null",
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_last_name_not_null",
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_store_id_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "staff_username_not_null",
                    expr: "username IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "staff_pkey",
                    columns: [
                        "staff_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "staff_address_id_fkey",
                    columns: [
                        "address_id",
                    ],
                    table: "address",
                    foreign_columns: [
                        "address_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "staff_store_id_fkey",
                    columns: [
                        "store_id",
                    ],
                    table: "store",
                    foreign_columns: [
                        "store_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "manager_staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "store_address_id_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "store_last_update_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "store_manager_staff_id_not_null",
                    expr: "manager_staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "store_store_id_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [
                Unique {
                    name: "idx_unq_manager_staff_id",
                    columns: [
                        "manager_staff_id",
                    ],
                    is_partial: false,
                },
            ],
            primary_key_constraints: [
                PrimaryKey {
                    name: "store_pkey",
                    columns: [
                        "store_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "store_address_id_fkey",
                    columns: [
                        "address_id",
                    ],
                    table: "address",
                    foreign_columns: [
                        "address_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "store_manager_staff_id_fkey",
                    columns: [
                        "manager_staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
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
                    is_identity: false,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                    is_identity: false,
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
                    is_identity: false,
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
                    is_identity: false,
                },
            ],
            check_constraints: [
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_amount_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_customer_id_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_date_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_payment_id_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_rental_id_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_staff_id_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [
                PrimaryKey {
                    name: "payment_pkey",
                    columns: [
                        "payment_id",
                    ],
                },
            ],
            reference_constraints: [
                References {
                    name: "payment_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
                References {
                    name: "payment_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        SetNull,
                    ),
                },
                References {
                    name: "payment_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
            ],
            exclusion_constraints: [],
        },
    ],
    enums: [
        EnumDef {
            values: [
                "G",
                "PG",
                "PG-13",
                "R",
                "NC-17",
            ],
            typename: "mpaa_rating",
            schema: "public",
        },
    ],
}