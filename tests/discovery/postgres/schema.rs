SELECT "table_name", "user_defined_type_schema", "user_defined_type_name" FROM "information_schema"."tables" WHERE "table_schema" = $1 AND "table_type" = $2, Values([String("public"), String("BASE TABLE")])

TableQueryResult { table_name: "actor", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "actor", of_type: None }
TableQueryResult { table_name: "film", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "film", of_type: None }
TableQueryResult { table_name: "payment_p2007_02", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment_p2007_02", of_type: None }
TableQueryResult { table_name: "payment_p2007_03", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment_p2007_03", of_type: None }
TableQueryResult { table_name: "payment_p2007_04", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment_p2007_04", of_type: None }
TableQueryResult { table_name: "payment_p2007_05", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment_p2007_05", of_type: None }
TableQueryResult { table_name: "payment_p2007_06", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment_p2007_06", of_type: None }
TableQueryResult { table_name: "payment_p2007_01", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment_p2007_01", of_type: None }
TableQueryResult { table_name: "address", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "address", of_type: None }
TableQueryResult { table_name: "category", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "category", of_type: None }
TableQueryResult { table_name: "city", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "city", of_type: None }
TableQueryResult { table_name: "country", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "country", of_type: None }
TableQueryResult { table_name: "customer", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "customer", of_type: None }
TableQueryResult { table_name: "film_actor", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "film_actor", of_type: None }
TableQueryResult { table_name: "film_category", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "film_category", of_type: None }
TableQueryResult { table_name: "inventory", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "inventory", of_type: None }
TableQueryResult { table_name: "language", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "language", of_type: None }
TableQueryResult { table_name: "rental", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "rental", of_type: None }
TableQueryResult { table_name: "staff", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "staff", of_type: None }
TableQueryResult { table_name: "store", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "store", of_type: None }
TableQueryResult { table_name: "payment", user_defined_type_schema: None, user_defined_type_name: None }
TableInfo { name: "payment", of_type: None }

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("actor")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("film")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment_p2007_02")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment_p2007_03")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment_p2007_04")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment_p2007_05")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment_p2007_06")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment_p2007_01")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("address")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("category")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("city")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("country")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("customer")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("film_actor")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("film_category")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("inventory")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("language")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("rental")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("staff")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("store")])

SELECT "column_name", "data_type", "column_default", "generation_expression", "is_nullable", "numeric_precision", "numeric_precision_radix", "numeric_scale" FROM "information_schema"."columns" WHERE "table_schema" = $1 AND "table_name" = $2, Values([String("public"), String("payment")])

ColumnQueryResult { column_name: "category_id", column_type: "integer", column_default: Some("nextval(\'category_category_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "category_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'category_category_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("category")])

ColumnQueryResult { column_name: "film_id", column_type: "integer", column_default: Some("nextval(\'film_film_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "film_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'film_film_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "title", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "title", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "description", column_type: "text", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "description", col_type: Unknown("text is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "release_year", column_type: "integer", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "release_year", col_type: Integer, default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "language_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "language_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "original_language_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "original_language_id", col_type: SmallInt, default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "rental_duration", column_type: "smallint", column_default: Some("3"), column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_duration", col_type: SmallInt, default: Some(ColumnExpression("3")), generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_rate", column_type: "numeric", column_default: Some("4.99"), column_generated: None, is_nullable: "NO", numeric_precision: Some(4), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "rental_rate", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(4), scale: Some(2) }), default: Some(ColumnExpression("4.99")), generated: None, not_null: None }
ColumnQueryResult { column_name: "length", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "length", col_type: SmallInt, default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "replacement_cost", column_type: "numeric", column_default: Some("19.99"), column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "replacement_cost", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: Some(ColumnExpression("19.99")), generated: None, not_null: None }
ColumnQueryResult { column_name: "rating", column_type: "USER-DEFINED", column_default: Some("\'G\'::mpaa_rating"), column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "rating", col_type: Unknown("USER is unknown or unimplemented"), default: Some(ColumnExpression("\'G\'::mpaa_rating")), generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }
ColumnQueryResult { column_name: "special_features", column_type: "ARRAY", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "special_features", col_type: Unknown("ARRAY is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "fulltext", column_type: "tsvector", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "fulltext", col_type: Unknown("tsvector is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("film")])

ColumnQueryResult { column_name: "city_id", column_type: "integer", column_default: Some("nextval(\'city_city_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "city_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'city_city_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "city", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "city", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "country_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "country_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("city")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment_p2007_03")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment_p2007_04")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment_p2007_06")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment_p2007_01")])

ColumnQueryResult { column_name: "address_id", column_type: "integer", column_default: Some("nextval(\'address_address_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "address_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'address_address_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "address", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "address", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "address2", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "address2", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "district", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "district", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "city_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "city_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "postal_code", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "postal_code", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "phone", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "phone", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("address")])

ColumnQueryResult { column_name: "country_id", column_type: "integer", column_default: Some("nextval(\'country_country_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "country_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'country_country_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "country", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "country", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("country")])

ColumnQueryResult { column_name: "actor_id", column_type: "integer", column_default: Some("nextval(\'actor_actor_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "actor_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'actor_actor_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "first_name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "first_name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("actor")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment_p2007_02")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment_p2007_05")])

ColumnQueryResult { column_name: "actor_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "actor_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "film_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "film_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("film_actor")])

ColumnQueryResult { column_name: "film_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "film_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "category_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "category_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("film_category")])

ColumnQueryResult { column_name: "inventory_id", column_type: "integer", column_default: Some("nextval(\'inventory_inventory_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "inventory_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'inventory_inventory_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "film_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "film_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "store_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "store_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("inventory")])

ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: Some("nextval(\'rental_rental_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'rental_rental_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "rental_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "inventory_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "inventory_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "return_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "return_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("rental")])

ColumnQueryResult { column_name: "staff_id", column_type: "integer", column_default: Some("nextval(\'staff_staff_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'staff_staff_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "first_name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "first_name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "address_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "address_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "email", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "email", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "store_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "store_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "active", column_type: "boolean", column_default: Some("true"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "active", col_type: Unknown("boolean is unknown or unimplemented"), default: Some(ColumnExpression("true")), generated: None, not_null: None }
ColumnQueryResult { column_name: "username", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "username", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "password", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "password", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }
ColumnQueryResult { column_name: "picture", column_type: "bytea", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "picture", col_type: Unknown("bytea is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("staff")])

ColumnQueryResult { column_name: "customer_id", column_type: "integer", column_default: Some("nextval(\'customer_customer_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'customer_customer_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "store_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "store_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "first_name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "first_name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_name", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "email", column_type: "character varying", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "email", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "address_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "address_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "activebool", column_type: "boolean", column_default: Some("true"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "activebool", col_type: Unknown("boolean is unknown or unimplemented"), default: Some(ColumnExpression("true")), generated: None, not_null: None }
ColumnQueryResult { column_name: "create_date", column_type: "date", column_default: Some("(\'now\'::text)::date"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "create_date", col_type: Unknown("date is unknown or unimplemented"), default: Some(ColumnExpression("(\'now\'::text)::date")), generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "YES", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: Some(NotNull) }
ColumnQueryResult { column_name: "active", column_type: "integer", column_default: None, column_generated: None, is_nullable: "YES", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "active", col_type: Integer, default: None, generated: None, not_null: Some(NotNull) }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("customer")])

ColumnQueryResult { column_name: "language_id", column_type: "integer", column_default: Some("nextval(\'language_language_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "language_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'language_language_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "name", column_type: "character", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "name", col_type: Unknown("character is unknown or unimplemented"), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("language")])

ColumnQueryResult { column_name: "store_id", column_type: "integer", column_default: Some("nextval(\'store_store_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "store_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'store_store_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "manager_staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "manager_staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "address_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "address_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "last_update", column_type: "timestamp without time zone", column_default: Some("now()"), column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "last_update", col_type: Unknown("timestamp is unknown or unimplemented"), default: Some(ColumnExpression("now()")), generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("store")])

ColumnQueryResult { column_name: "payment_id", column_type: "integer", column_default: Some("nextval(\'payment_payment_id_seq\'::regclass)"), column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "payment_id", col_type: Integer, default: Some(ColumnExpression("nextval(\'payment_payment_id_seq\'::regclass)")), generated: None, not_null: None }
ColumnQueryResult { column_name: "customer_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "customer_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "staff_id", column_type: "smallint", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(16), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "staff_id", col_type: SmallInt, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "rental_id", column_type: "integer", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(32), numeric_precision_radix: Some(2), numeric_scale: Some(0) }
ColumnInfo { name: "rental_id", col_type: Integer, default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "amount", column_type: "numeric", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: Some(5), numeric_precision_radix: Some(10), numeric_scale: Some(2) }
ColumnInfo { name: "amount", col_type: Numeric(ArbitraryPrecisionNumericAttr { precision: Some(5), scale: Some(2) }), default: None, generated: None, not_null: None }
ColumnQueryResult { column_name: "payment_date", column_type: "timestamp without time zone", column_default: None, column_generated: None, is_nullable: "NO", numeric_precision: None, numeric_precision_radix: None, numeric_scale: None }
ColumnInfo { name: "payment_date", col_type: Unknown("timestamp is unknown or unimplemented"), default: None, generated: None, not_null: None }

SELECT "table_constraints"."constraint_schema", "table_constraints"."constraint_name", "table_constraints"."table_schema", "table_constraints"."table_name", "table_constraints"."constraint_type", "table_constraints"."is_deferrable", "table_constraints"."initially_deferred", "check_constraints"."check_clause", "key_column_usage"."column_name", "key_column_usage"."ordinal_position", "key_column_usage"."position_in_unique_constraint", "referential_constraints_subquery"."unique_constraint_schema", "referential_constraints_subquery"."unique_constraint_name", "referential_constraints_subquery"."match_option", "referential_constraints_subquery"."update_rule", "referential_constraints_subquery"."delete_rule", "referential_constraints_subquery"."table_name", "referential_constraints_subquery"."column_name" FROM "information_schema"."table_constraints" LEFT JOIN "information_schema"."check_constraints" ON "table_constraints"."constraint_name" = "check_constraints"."constraint_name" LEFT JOIN "information_schema"."key_column_usage" ON "table_constraints"."constraint_name" = "key_column_usage"."constraint_name" LEFT JOIN (SELECT "referential_constraints"."constraint_name", "referential_constraints"."unique_constraint_schema", "referential_constraints"."unique_constraint_name", "referential_constraints"."match_option", "referential_constraints"."update_rule", "referential_constraints"."delete_rule", "key_column_usage"."table_name", "key_column_usage"."column_name" FROM "information_schema"."referential_constraints" LEFT JOIN "information_schema"."key_column_usage" ON "referential_constraints"."unique_constraint_name" = "key_column_usage"."constraint_name") AS "referential_constraints_subquery" ON "table_constraints"."constraint_name" = "referential_constraints_subquery"."constraint_name" WHERE "table_constraints"."table_schema" = $1 AND "table_constraints"."table_name" = $2 ORDER BY "table_constraints"."constraint_name" ASC, "key_column_usage"."ordinal_position" ASC, "referential_constraints_subquery"."unique_constraint_name" ASC, "referential_constraints_subquery"."constraint_name" ASC, Values([String("public"), String("payment")])

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_10_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("replacement_cost IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_12_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_14_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("fulltext IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_1_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("film_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_2_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("title IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_5_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("language_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_7_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_duration IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25011_8_not_null", table_schema: "public", table_name: "film", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_rate IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_language_id_fkey", table_schema: "public", table_name: "film", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("language_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("language_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("language"), referential_key_column_name: Some("language_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_original_language_id_fkey", table_schema: "public", table_name: "film", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("original_language_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("language_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("language"), referential_key_column_name: Some("language_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_pkey", table_schema: "public", table_name: "film", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("film_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "replacement_cost IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
Check(Check { expr: "fulltext IS NOT NULL", no_inherit: false })
Check(Check { expr: "film_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "title IS NOT NULL", no_inherit: false })
Check(Check { expr: "language_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_duration IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_rate IS NOT NULL", no_inherit: false })
References(References { columns: ["language_id"], table: "language", foreign_columns: ["language_id"] })
References(References { columns: ["original_language_id"], table: "language", foreign_columns: ["language_id"] })
PrimaryKey(PrimaryKey(["film_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25116_1_not_null", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25116_2_not_null", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25116_3_not_null", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25116_4_not_null", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25116_5_not_null", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25116_6_not_null", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_04_customer_id_fkey", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_04_payment_date_check", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("(((payment_date >= \'2007-04-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-05-01 00:00:00\'::timestamp without time zone)))"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_04_rental_id_fkey", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_04_staff_id_fkey", table_schema: "public", table_name: "payment_p2007_04", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
Check(Check { expr: "(((payment_date >= \'2007-04-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-05-01 00:00:00\'::timestamp without time zone)))", no_inherit: false })
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25004_1_not_null", table_schema: "public", table_name: "category", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("category_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25004_2_not_null", table_schema: "public", table_name: "category", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25004_3_not_null", table_schema: "public", table_name: "category", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "category_pkey", table_schema: "public", table_name: "category", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("category_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "category_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "name IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
PrimaryKey(PrimaryKey(["category_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25106_1_not_null", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25106_2_not_null", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25106_3_not_null", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25106_4_not_null", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25106_5_not_null", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25106_6_not_null", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_02_customer_id_fkey", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_02_payment_date_check", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("(((payment_date >= \'2007-02-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-03-01 00:00:00\'::timestamp without time zone)))"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_02_rental_id_fkey", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_02_staff_id_fkey", table_schema: "public", table_name: "payment_p2007_02", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
Check(Check { expr: "(((payment_date >= \'2007-02-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-03-01 00:00:00\'::timestamp without time zone)))", no_inherit: false })
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25121_1_not_null", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25121_2_not_null", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25121_3_not_null", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25121_4_not_null", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25121_5_not_null", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25121_6_not_null", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_05_customer_id_fkey", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_05_payment_date_check", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("(((payment_date >= \'2007-05-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-06-01 00:00:00\'::timestamp without time zone)))"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_05_rental_id_fkey", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_05_staff_id_fkey", table_schema: "public", table_name: "payment_p2007_05", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
Check(Check { expr: "(((payment_date >= \'2007-05-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-06-01 00:00:00\'::timestamp without time zone)))", no_inherit: false })
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25101_1_not_null", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25101_2_not_null", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25101_3_not_null", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25101_4_not_null", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25101_5_not_null", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25101_6_not_null", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_01_customer_id_fkey", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_01_payment_date_check", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("(((payment_date >= \'2007-01-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-02-01 00:00:00\'::timestamp without time zone)))"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_01_rental_id_fkey", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_01_staff_id_fkey", table_schema: "public", table_name: "payment_p2007_01", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
Check(Check { expr: "(((payment_date >= \'2007-01-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-02-01 00:00:00\'::timestamp without time zone)))", no_inherit: false })
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_24980_1_not_null", table_schema: "public", table_name: "actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("actor_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_24980_2_not_null", table_schema: "public", table_name: "actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("first_name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_24980_3_not_null", table_schema: "public", table_name: "actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_24980_4_not_null", table_schema: "public", table_name: "actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "actor_pkey", table_schema: "public", table_name: "actor", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("actor_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "actor_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "first_name IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_name IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
PrimaryKey(PrimaryKey(["actor_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25038_1_not_null", table_schema: "public", table_name: "address", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("address_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25038_2_not_null", table_schema: "public", table_name: "address", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("address IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25038_4_not_null", table_schema: "public", table_name: "address", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("district IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25038_5_not_null", table_schema: "public", table_name: "address", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("city_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25038_7_not_null", table_schema: "public", table_name: "address", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("phone IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25038_8_not_null", table_schema: "public", table_name: "address", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "address_city_id_fkey", table_schema: "public", table_name: "address", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("city_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("city_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("city"), referential_key_column_name: Some("city_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "address_pkey", table_schema: "public", table_name: "address", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("address_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "address_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "address IS NOT NULL", no_inherit: false })
Check(Check { expr: "district IS NOT NULL", no_inherit: false })
Check(Check { expr: "city_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "phone IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["city_id"], table: "city", foreign_columns: ["city_id"] })
PrimaryKey(PrimaryKey(["address_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25111_1_not_null", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25111_2_not_null", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25111_3_not_null", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25111_4_not_null", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25111_5_not_null", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25111_6_not_null", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_03_customer_id_fkey", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_03_payment_date_check", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("(((payment_date >= \'2007-03-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-04-01 00:00:00\'::timestamp without time zone)))"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_03_rental_id_fkey", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_03_staff_id_fkey", table_schema: "public", table_name: "payment_p2007_03", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
Check(Check { expr: "(((payment_date >= \'2007-03-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-04-01 00:00:00\'::timestamp without time zone)))", no_inherit: false })
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25052_1_not_null", table_schema: "public", table_name: "country", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("country_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25052_2_not_null", table_schema: "public", table_name: "country", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("country IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25052_3_not_null", table_schema: "public", table_name: "country", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "country_pkey", table_schema: "public", table_name: "country", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("country_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "country_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "country IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
PrimaryKey(PrimaryKey(["country_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25126_1_not_null", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25126_2_not_null", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25126_3_not_null", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25126_4_not_null", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25126_5_not_null", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25126_6_not_null", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_06_customer_id_fkey", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_06_payment_date_check", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("(((payment_date >= \'2007-06-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-07-01 00:00:00\'::timestamp without time zone)))"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_06_rental_id_fkey", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_p2007_06_staff_id_fkey", table_schema: "public", table_name: "payment_p2007_06", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
Check(Check { expr: "(((payment_date >= \'2007-06-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-07-01 00:00:00\'::timestamp without time zone)))", no_inherit: false })
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25045_1_not_null", table_schema: "public", table_name: "city", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("city_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25045_2_not_null", table_schema: "public", table_name: "city", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("city IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25045_3_not_null", table_schema: "public", table_name: "city", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("country_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25045_4_not_null", table_schema: "public", table_name: "city", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "city_country_id_fkey", table_schema: "public", table_name: "city", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("country_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("country_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("country"), referential_key_column_name: Some("country_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "city_pkey", table_schema: "public", table_name: "city", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("city_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "city_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "city IS NOT NULL", no_inherit: false })
Check(Check { expr: "country_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["country_id"], table: "country", foreign_columns: ["country_id"] })
PrimaryKey(PrimaryKey(["city_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25023_1_not_null", table_schema: "public", table_name: "film_actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("actor_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25023_2_not_null", table_schema: "public", table_name: "film_actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("film_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25023_3_not_null", table_schema: "public", table_name: "film_actor", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_actor_actor_id_fkey", table_schema: "public", table_name: "film_actor", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("actor_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("actor_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("actor"), referential_key_column_name: Some("actor_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_actor_film_id_fkey", table_schema: "public", table_name: "film_actor", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("film_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("film_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("film"), referential_key_column_name: Some("film_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_actor_pkey", table_schema: "public", table_name: "film_actor", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("actor_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_actor_pkey", table_schema: "public", table_name: "film_actor", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("film_id"), ordinal_position: Some(2), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "actor_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "film_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["actor_id"], table: "actor", foreign_columns: ["actor_id"] })
References(References { columns: ["film_id"], table: "film", foreign_columns: ["film_id"] })
PrimaryKey(PrimaryKey(["actor_id", "film_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25078_1_not_null", table_schema: "public", table_name: "inventory", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("inventory_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25078_2_not_null", table_schema: "public", table_name: "inventory", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("film_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25078_3_not_null", table_schema: "public", table_name: "inventory", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("store_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25078_4_not_null", table_schema: "public", table_name: "inventory", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "inventory_film_id_fkey", table_schema: "public", table_name: "inventory", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("film_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("film_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("film"), referential_key_column_name: Some("film_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "inventory_pkey", table_schema: "public", table_name: "inventory", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("inventory_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "inventory_store_id_fkey", table_schema: "public", table_name: "inventory", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("store_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("store_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("store"), referential_key_column_name: Some("store_id") }

Check(Check { expr: "inventory_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "film_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "store_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["film_id"], table: "film", foreign_columns: ["film_id"] })
PrimaryKey(PrimaryKey(["inventory_id"]))
References(References { columns: ["store_id"], table: "store", foreign_columns: ["store_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25027_1_not_null", table_schema: "public", table_name: "film_category", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("film_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25027_2_not_null", table_schema: "public", table_name: "film_category", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("category_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25027_3_not_null", table_schema: "public", table_name: "film_category", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_category_category_id_fkey", table_schema: "public", table_name: "film_category", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("category_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("category_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("category"), referential_key_column_name: Some("category_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_category_film_id_fkey", table_schema: "public", table_name: "film_category", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("film_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("film_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("film"), referential_key_column_name: Some("film_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_category_pkey", table_schema: "public", table_name: "film_category", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("film_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "film_category_pkey", table_schema: "public", table_name: "film_category", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("category_id"), ordinal_position: Some(2), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "film_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "category_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["category_id"], table: "category", foreign_columns: ["category_id"] })
References(References { columns: ["film_id"], table: "film", foreign_columns: ["film_id"] })
PrimaryKey(PrimaryKey(["film_id", "category_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25133_1_not_null", table_schema: "public", table_name: "rental", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25133_2_not_null", table_schema: "public", table_name: "rental", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25133_3_not_null", table_schema: "public", table_name: "rental", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("inventory_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25133_4_not_null", table_schema: "public", table_name: "rental", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25133_6_not_null", table_schema: "public", table_name: "rental", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25133_7_not_null", table_schema: "public", table_name: "rental", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "rental_customer_id_fkey", table_schema: "public", table_name: "rental", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "rental_inventory_id_fkey", table_schema: "public", table_name: "rental", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("inventory_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("inventory_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("inventory"), referential_key_column_name: Some("inventory_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "rental_pkey", table_schema: "public", table_name: "rental", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "rental_staff_id_fkey", table_schema: "public", table_name: "rental", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_date IS NOT NULL", no_inherit: false })
Check(Check { expr: "inventory_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
References(References { columns: ["inventory_id"], table: "inventory", foreign_columns: ["inventory_id"] })
PrimaryKey(PrimaryKey(["rental_id"]))
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_10_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_1_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_2_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("first_name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_3_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_4_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("address_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_6_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("store_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_7_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("active IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25145_8_not_null", table_schema: "public", table_name: "staff", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("username IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "staff_address_id_fkey", table_schema: "public", table_name: "staff", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("address_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("address_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("address"), referential_key_column_name: Some("address_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "staff_pkey", table_schema: "public", table_name: "staff", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "staff_store_id_fkey", table_schema: "public", table_name: "staff", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("store_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("store_pkey"), match_option: Some("NONE"), update_rule: Some("NO ACTION"), delete_rule: Some("NO ACTION"), referential_key_table_name: Some("store"), referential_key_column_name: Some("store_id") }

Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "first_name IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_name IS NOT NULL", no_inherit: false })
Check(Check { expr: "address_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "store_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "active IS NOT NULL", no_inherit: false })
Check(Check { expr: "username IS NOT NULL", no_inherit: false })
References(References { columns: ["address_id"], table: "address", foreign_columns: ["address_id"] })
PrimaryKey(PrimaryKey(["staff_id"]))
References(References { columns: ["store_id"], table: "store", foreign_columns: ["store_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_1_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_2_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("store_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_3_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("first_name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_4_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_6_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("address_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_7_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("activebool IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25059_8_not_null", table_schema: "public", table_name: "customer", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("create_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "customer_address_id_fkey", table_schema: "public", table_name: "customer", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("address_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("address_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("address"), referential_key_column_name: Some("address_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "customer_pkey", table_schema: "public", table_name: "customer", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "customer_store_id_fkey", table_schema: "public", table_name: "customer", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("store_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("store_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("store"), referential_key_column_name: Some("store_id") }

Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "store_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "first_name IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_name IS NOT NULL", no_inherit: false })
Check(Check { expr: "address_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "activebool IS NOT NULL", no_inherit: false })
Check(Check { expr: "create_date IS NOT NULL", no_inherit: false })
References(References { columns: ["address_id"], table: "address", foreign_columns: ["address_id"] })
PrimaryKey(PrimaryKey(["customer_id"]))
References(References { columns: ["store_id"], table: "store", foreign_columns: ["store_id"] })

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25085_1_not_null", table_schema: "public", table_name: "language", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("language_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25085_2_not_null", table_schema: "public", table_name: "language", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("name IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25085_3_not_null", table_schema: "public", table_name: "language", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "language_pkey", table_schema: "public", table_name: "language", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("language_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "language_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "name IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
PrimaryKey(PrimaryKey(["language_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25156_1_not_null", table_schema: "public", table_name: "store", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("store_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25156_2_not_null", table_schema: "public", table_name: "store", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("manager_staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25156_3_not_null", table_schema: "public", table_name: "store", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("address_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25156_4_not_null", table_schema: "public", table_name: "store", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("last_update IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "store_address_id_fkey", table_schema: "public", table_name: "store", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("address_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("address_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("address"), referential_key_column_name: Some("address_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "store_manager_staff_id_fkey", table_schema: "public", table_name: "store", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("manager_staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "store_pkey", table_schema: "public", table_name: "store", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("store_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }

Check(Check { expr: "store_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "manager_staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "address_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "last_update IS NOT NULL", no_inherit: false })
References(References { columns: ["address_id"], table: "address", foreign_columns: ["address_id"] })
References(References { columns: ["manager_staff_id"], table: "staff", foreign_columns: ["staff_id"] })
PrimaryKey(PrimaryKey(["store_id"]))

TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25097_1_not_null", table_schema: "public", table_name: "payment", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25097_2_not_null", table_schema: "public", table_name: "payment", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("customer_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25097_3_not_null", table_schema: "public", table_name: "payment", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("staff_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25097_4_not_null", table_schema: "public", table_name: "payment", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("rental_id IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25097_5_not_null", table_schema: "public", table_name: "payment", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("amount IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "2200_25097_6_not_null", table_schema: "public", table_name: "payment", constraint_type: "CHECK", is_deferrable: "NO", initially_deferred: "NO", check_clause: Some("payment_date IS NOT NULL"), column_name: None, ordinal_position: None, position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_customer_id_fkey", table_schema: "public", table_name: "payment", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("customer_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("customer_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("customer"), referential_key_column_name: Some("customer_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_pkey", table_schema: "public", table_name: "payment", constraint_type: "PRIMARY KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("payment_id"), ordinal_position: Some(1), position_in_unique_constraint: None, unique_constraint_schema: None, unique_constraint_name: None, match_option: None, update_rule: None, delete_rule: None, referential_key_table_name: None, referential_key_column_name: None }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_rental_id_fkey", table_schema: "public", table_name: "payment", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("rental_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("rental_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("SET NULL"), referential_key_table_name: Some("rental"), referential_key_column_name: Some("rental_id") }
TableConstraintsQueryResult { constraint_schema: "public", constraint_name: "payment_staff_id_fkey", table_schema: "public", table_name: "payment", constraint_type: "FOREIGN KEY", is_deferrable: "NO", initially_deferred: "NO", check_clause: None, column_name: Some("staff_id"), ordinal_position: Some(1), position_in_unique_constraint: Some(1), unique_constraint_schema: Some("public"), unique_constraint_name: Some("staff_pkey"), match_option: Some("NONE"), update_rule: Some("CASCADE"), delete_rule: Some("RESTRICT"), referential_key_table_name: Some("staff"), referential_key_column_name: Some("staff_id") }

Check(Check { expr: "payment_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "customer_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "staff_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "rental_id IS NOT NULL", no_inherit: false })
Check(Check { expr: "amount IS NOT NULL", no_inherit: false })
Check(Check { expr: "payment_date IS NOT NULL", no_inherit: false })
References(References { columns: ["customer_id"], table: "customer", foreign_columns: ["customer_id"] })
PrimaryKey(PrimaryKey(["payment_id"]))
References(References { columns: ["rental_id"], table: "rental", foreign_columns: ["rental_id"] })
References(References { columns: ["staff_id"], table: "staff", foreign_columns: ["staff_id"] })

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
                    not_null: None,
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
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
            unique_keys: [],
            references: [],
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "title",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "description",
                    col_type: Unknown(
                        "text is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "release_year",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "language_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "original_language_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "length",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "rating",
                    col_type: Unknown(
                        "USER is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "\'G\'::mpaa_rating",
                        ),
                    ),
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
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
                    name: "special_features",
                    col_type: Unknown(
                        "ARRAY is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "fulltext",
                    col_type: Unknown(
                        "tsvector is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
                    expr: "(((payment_date >= \'2007-02-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-03-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
                    expr: "(((payment_date >= \'2007-03-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-04-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
                    expr: "(((payment_date >= \'2007-04-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-05-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
                    expr: "(((payment_date >= \'2007-05-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-06-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
                    expr: "(((payment_date >= \'2007-06-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-07-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
                    expr: "(((payment_date >= \'2007-01-01 00:00:00\'::timestamp without time zone) AND (payment_date < \'2007-02-01 00:00:00\'::timestamp without time zone)))",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "address",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "address2",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "district",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "city_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "postal_code",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "phone",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [],
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "city",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "country_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "country",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [],
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "email",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "activebool",
                    col_type: Unknown(
                        "boolean is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "true",
                        ),
                    ),
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "create_date",
                    col_type: Unknown(
                        "date is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "(\'now\'::text)::date",
                        ),
                    ),
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
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
                    name: "active",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
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
                    expr: "film_id IS NOT NULL",
                    no_inherit: false,
                },
                Check {
                    expr: "last_update IS NOT NULL",
                    no_inherit: false,
                },
            ],
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "category_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "film_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "store_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [],
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "inventory_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "return_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "first_name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_name",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "email",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "active",
                    col_type: Unknown(
                        "boolean is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "true",
                        ),
                    ),
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "username",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "password",
                    col_type: Unknown(
                        "character is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: Some(
                        NotNull,
                    ),
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
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
                    name: "picture",
                    col_type: Unknown(
                        "bytea is unknown or unimplemented",
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "manager_staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "address_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "last_update",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: Some(
                        ColumnExpression(
                            "now()",
                        ),
                    ),
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "customer_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "staff_id",
                    col_type: SmallInt,
                    default: None,
                    generated: None,
                    not_null: None,
                },
                ColumnInfo {
                    name: "rental_id",
                    col_type: Integer,
                    default: None,
                    generated: None,
                    not_null: None,
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
                    not_null: None,
                },
                ColumnInfo {
                    name: "payment_date",
                    col_type: Unknown(
                        "timestamp is unknown or unimplemented",
                    ),
                    default: None,
                    generated: None,
                    not_null: None,
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
            unique_keys: [],
            references: [
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
            of_type: None,
        },
    ],
}
