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
                            "nextval(\'actor_actor_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    name: "2200_16388_1_not_null",
                    expr: "actor_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16388_2_not_null",
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16388_3_not_null",
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16388_4_not_null",
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
                name: "film",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "film_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'film_film_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                        "USER-DEFINED",
                    ),
                    default: Some(
                        ColumnExpression(
                            "\'G\'::mpaa_rating",
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
                    name: "2200_16419_10_not_null",
                    expr: "replacement_cost IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_12_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_14_not_null",
                    expr: "fulltext IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_1_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_2_not_null",
                    expr: "title IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_5_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_7_not_null",
                    expr: "rental_duration IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16419_8_not_null",
                    expr: "rental_rate IS NOT NULL",
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
                name: "payment_p2007_02",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16514_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16514_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16514_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16514_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16514_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16514_6_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_p2007_02_payment_date_check",
                    expr: "(((payment_date >= \'2007-02-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-03-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    name: "payment_p2007_02_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_02_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_02_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
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
                name: "payment_p2007_03",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16519_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16519_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16519_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16519_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16519_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16519_6_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_p2007_03_payment_date_check",
                    expr: "(((payment_date >= \'2007-03-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-04-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    name: "payment_p2007_03_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_03_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_03_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
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
                name: "payment_p2007_04",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16524_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16524_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16524_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16524_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16524_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16524_6_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_p2007_04_payment_date_check",
                    expr: "(((payment_date >= \'2007-04-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-05-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    name: "payment_p2007_04_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_04_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_04_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
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
                name: "payment_p2007_05",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16529_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16529_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16529_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16529_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16529_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16529_6_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_p2007_05_payment_date_check",
                    expr: "(((payment_date >= \'2007-05-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-06-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    name: "payment_p2007_05_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_05_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_05_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
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
                name: "payment_p2007_06",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16534_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16534_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16534_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16534_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16534_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16534_6_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_p2007_06_payment_date_check",
                    expr: "(((payment_date >= \'2007-06-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-07-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    name: "payment_p2007_06_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_06_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_06_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
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
                name: "payment_p2007_01",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "payment_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16509_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16509_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16509_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16509_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16509_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16509_6_not_null",
                    expr: "payment_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "payment_p2007_01_payment_date_check",
                    expr: "(((payment_date >= \'2007-01-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-02-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
            primary_key_constraints: [],
            reference_constraints: [
                References {
                    name: "payment_p2007_01_customer_id_fkey",
                    columns: [
                        "customer_id",
                    ],
                    table: "customer",
                    foreign_columns: [
                        "customer_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_01_rental_id_fkey",
                    columns: [
                        "rental_id",
                    ],
                    table: "rental",
                    foreign_columns: [
                        "rental_id",
                    ],
                    on_update: Some(
                        NoAction,
                    ),
                    on_delete: Some(
                        NoAction,
                    ),
                },
                References {
                    name: "payment_p2007_01_staff_id_fkey",
                    columns: [
                        "staff_id",
                    ],
                    table: "staff",
                    foreign_columns: [
                        "staff_id",
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
                name: "address",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "address_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'address_address_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    name: "2200_16446_1_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16446_2_not_null",
                    expr: "address IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16446_4_not_null",
                    expr: "district IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16446_5_not_null",
                    expr: "city_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16446_7_not_null",
                    expr: "phone IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16446_8_not_null",
                    expr: "last_update IS NOT NULL",
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
                            "nextval(\'category_category_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    name: "2200_16412_1_not_null",
                    expr: "category_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16412_2_not_null",
                    expr: "name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16412_3_not_null",
                    expr: "last_update IS NOT NULL",
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
                            "nextval(\'city_city_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    name: "2200_16453_1_not_null",
                    expr: "city_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16453_2_not_null",
                    expr: "city IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16453_3_not_null",
                    expr: "country_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16453_4_not_null",
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
                name: "country",
                of_type: None,
            },
            columns: [
                ColumnInfo {
                    name: "country_id",
                    col_type: Integer,
                    default: Some(
                        ColumnExpression(
                            "nextval(\'country_country_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    name: "2200_16460_1_not_null",
                    expr: "country_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16460_2_not_null",
                    expr: "country IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16460_3_not_null",
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
                            "nextval(\'customer_customer_id_seq\'::regclass)",
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
                            "(\'now\'::text)::date",
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
                    name: "2200_16467_1_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16467_2_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16467_3_not_null",
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16467_4_not_null",
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16467_6_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16467_7_not_null",
                    expr: "activebool IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16467_8_not_null",
                    expr: "create_date IS NOT NULL",
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
                    name: "2200_16431_1_not_null",
                    expr: "actor_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16431_2_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16431_3_not_null",
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
                References {
                    name: "film_actor_film_id_fkey",
                    columns: [
                        "film_id",
                    ],
                    table: "film",
                    foreign_columns: [
                        "film_id",
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
                    name: "2200_16435_1_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16435_2_not_null",
                    expr: "category_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16435_3_not_null",
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
                References {
                    name: "film_category_film_id_fkey",
                    columns: [
                        "film_id",
                    ],
                    table: "film",
                    foreign_columns: [
                        "film_id",
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
                            "nextval(\'inventory_inventory_id_seq\'::regclass)",
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
                    name: "2200_16486_1_not_null",
                    expr: "inventory_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16486_2_not_null",
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16486_3_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16486_4_not_null",
                    expr: "last_update IS NOT NULL",
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
                    name: "inventory_film_id_fkey",
                    columns: [
                        "film_id",
                    ],
                    table: "film",
                    foreign_columns: [
                        "film_id",
                    ],
                    on_update: Some(
                        Cascade,
                    ),
                    on_delete: Some(
                        Restrict,
                    ),
                },
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
                            "nextval(\'language_language_id_seq\'::regclass)",
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
                    name: "2200_16493_1_not_null",
                    expr: "language_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16493_2_not_null",
                    expr: "name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16493_3_not_null",
                    expr: "last_update IS NOT NULL",
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
                            "nextval(\'rental_rental_id_seq\'::regclass)",
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
                    name: "2200_16541_1_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16541_2_not_null",
                    expr: "rental_date IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16541_3_not_null",
                    expr: "inventory_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16541_4_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16541_6_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16541_7_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
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
                            "nextval(\'staff_staff_id_seq\'::regclass)",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    name: "2200_16553_10_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_1_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_2_not_null",
                    expr: "first_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_3_not_null",
                    expr: "last_name IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_4_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_6_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_7_not_null",
                    expr: "active IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16553_8_not_null",
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
                            "nextval(\'store_store_id_seq\'::regclass)",
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
                    name: "2200_16564_1_not_null",
                    expr: "store_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16564_2_not_null",
                    expr: "manager_staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16564_3_not_null",
                    expr: "address_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16564_4_not_null",
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            not_null_constraints: [],
            unique_constraints: [],
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
                            "nextval(\'payment_payment_id_seq\'::regclass)",
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
                    name: "2200_16505_1_not_null",
                    expr: "payment_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16505_2_not_null",
                    expr: "customer_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16505_3_not_null",
                    expr: "staff_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16505_4_not_null",
                    expr: "rental_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16505_5_not_null",
                    expr: "amount IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    name: "2200_16505_6_not_null",
                    expr: "payment_date IS NOT NULL",
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
}
