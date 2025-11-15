use pretty_assertions::assert_eq;
use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sea_schema::sea_query::TableRef;
use sea_schema::sea_query::{
    ColumnDef, ColumnType, Expr, ForeignKey, ForeignKeyAction, Index, PostgresQueryBuilder, Table,
    TableCreateStatement, TableName, extension::postgres::Type,
};
use sqlx::{PgPool, Pool, Postgres};
use std::collections::HashMap;

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();

    let url = std::env::var("DATABASE_URL_LIVE")
        .unwrap_or_else(|_| "postgres://sea:sea@localhost".to_owned());

    let connection = setup(&url, "sea-schema").await;
    let mut executor = connection.acquire().await.unwrap();

    sqlx::query("CREATE EXTENSION IF NOT EXISTS citext")
        .execute(&mut *executor)
        .await
        .unwrap();

    let create_enum_stmt = Type::create()
        .as_enum("crazy_enum")
        .values(vec![
            "Astro0%00%8987,.!@#$%^&*()_-+=[]{}\\|.<>/? ``",
            "Biology",
            "Chemistry",
            "Math",
            "Physics",
        ])
        .to_string(PostgresQueryBuilder);

    sqlx::query(&create_enum_stmt)
        .execute(&mut *executor)
        .await
        .unwrap();

    let tbl_create_stmts = vec![
        create_bakery_table(),
        create_baker_table(),
        create_customer_table(),
        create_order_table(),
        create_cake_table(),
        create_cakes_bakers_table(),
        create_lineitem_table(),
        create_collection_table(),
        create_parent_table(),
        create_child_table(),
        create_db_types_table(),
        create_fkey_parent_table(),
        create_fkey_child_table(),
        create_fkey_parent_no_uniq_table(),
        create_fkey_child_no_uniq_table(),
        create_fkey_pair_parent_table(),
        create_fkey_pair_child_table(),
    ];

    for tbl_create_stmt in tbl_create_stmts.iter() {
        let sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        println!("{sql};");
        println!();
        sqlx::query(&sql).execute(&mut *executor).await.unwrap();

        if sql.starts_with(r#"CREATE TABLE "fkey_parent_table_no_uniq""#) {
            // add unique index, but not constraint
            let sql = Index::create()
                .table("fkey_parent_table_no_uniq")
                .name("idx-fkey_parent_table_no_uniq-u")
                .col("u")
                .unique()
                .to_string(PostgresQueryBuilder);
            sqlx::query(&sql).execute(&mut *executor).await.unwrap();
        }
    }

    let schema_discovery = SchemaDiscovery::new(connection, "public");

    let schema = schema_discovery
        .discover()
        .await
        .expect("Error discovering schema");

    println!("{schema:#?}");

    let map: HashMap<String, TableDef> = schema
        .tables
        .iter()
        .map(|table| (table.info.name.clone(), table.clone()))
        .collect();

    for tbl_create_stmt in tbl_create_stmts.into_iter() {
        let expected_sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        let tbl_name = match tbl_create_stmt.get_table_name() {
            Some(TableRef::Table(TableName(_, tbl), _)) => tbl.to_string(),
            _ => unimplemented!(),
        };
        let table = map.get(&tbl_name).unwrap();
        let sql = table.write().to_string(PostgresQueryBuilder);
        println!("Expected SQL:");
        println!("{expected_sql};");
        println!("Generated SQL:");
        println!("{sql};");
        println!();

        if sql.starts_with(r#"CREATE TABLE "fkey_parent_table_no_uniq""#) {
            // the fk promoted the unique index into constraint
            let expected_sql =
                create_fkey_parent_no_uniq_table_rediscover().to_string(PostgresQueryBuilder);
            assert_eq!(expected_sql, sql);
        } else {
            assert_eq!(expected_sql, sql);
        }
    }

    let enum_defs = schema_discovery
        .discover_enums()
        .await
        .expect("Error discovering enums");

    dbg!(&enum_defs);

    let enum_create_statements: Vec<String> = enum_defs
        .into_iter()
        .map(|enum_def| enum_def.write().to_string(PostgresQueryBuilder))
        .collect();

    dbg!(&create_enum_stmt);
    dbg!(&enum_create_statements);

    assert_eq!(create_enum_stmt, enum_create_statements[0]);
}

async fn setup(base_url: &str, db_name: &str) -> Pool<Postgres> {
    let url = format!("{base_url}/postgres");
    let mut connection = PgPool::connect(&url)
        .await
        .unwrap()
        .acquire()
        .await
        .unwrap();

    let _drop_db_result = sqlx::query(&format!("DROP DATABASE IF EXISTS \"{db_name}\";"))
        .bind(db_name)
        .execute(&mut *connection)
        .await
        .unwrap();

    let _create_db_result = sqlx::query(&format!("CREATE DATABASE \"{db_name}\";"))
        .execute(&mut *connection)
        .await
        .unwrap();

    let url = format!("{base_url}/{db_name}");
    PgPool::connect(&url).await.unwrap()
}

fn create_bakery_table() -> TableCreateStatement {
    Table::create()
        .table("bakery")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("profit_margin").double())
        .col(ColumnDef::new("crazy_enum_col").custom("crazy_enum"))
        .primary_key(Index::create().primary().name("bakery_pkey").col("id"))
        .to_owned()
}

fn create_baker_table() -> TableCreateStatement {
    Table::create()
        .table("baker")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("contact_details").json())
        .col(ColumnDef::new("bakery_id").integer())
        .primary_key(Index::create().primary().name("baker_pkey").col("id"))
        .foreign_key(
            ForeignKey::create()
                .name("FK_baker_bakery")
                .from("baker", "bakery_id")
                .to("bakery", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_customer_table() -> TableCreateStatement {
    Table::create()
        .table("customer")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("notes").text())
        .primary_key(Index::create().primary().name("customer_pkey").col("id"))
        .to_owned()
}

fn create_order_table() -> TableCreateStatement {
    Table::create()
        .table("order")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("total").decimal_len(19, 4))
        .col(ColumnDef::new("bakery_id").integer().not_null())
        .col(ColumnDef::new("customer_id").integer().not_null())
        .col(
            ColumnDef::new("placed_at")
                .date_time()
                .not_null()
                .default(Expr::current_timestamp()),
        )
        .col(
            ColumnDef::new("updated")
                .date_time()
                .not_null()
                .extra("DEFAULT '2023-06-07 16:24:00'::timestamp without time zone"),
        )
        .col(
            ColumnDef::new("net_weight")
                .double()
                .not_null()
                .default(10.05),
        )
        .col(ColumnDef::new("priority").integer().not_null().default(5))
        .primary_key(Index::create().primary().name("order_pkey").col("id"))
        .foreign_key(
            ForeignKey::create()
                .name("FK_order_bakery")
                .from("order", "bakery_id")
                .to("bakery", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_order_customer")
                .from("order", "customer_id")
                .to("customer", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_lineitem_table() -> TableCreateStatement {
    Table::create()
        .table("lineitem")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("price").decimal_len(19, 4))
        .col(ColumnDef::new("quantity").integer())
        .col(ColumnDef::new("order_id").integer().not_null())
        .col(ColumnDef::new("cake_id").integer().not_null())
        .primary_key(Index::create().primary().name("lineitem_pkey").col("id"))
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id")
                .col("cake_id"),
        )
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id_order_id")
                .col("cake_id")
                .col("order_id"),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_lineitem_cake")
                .from("lineitem", "cake_id")
                .to("cake", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_lineitem_order")
                .from("lineitem", "order_id")
                .to("order", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_cakes_bakers_table() -> TableCreateStatement {
    Table::create()
        .table("cakes_bakers")
        .col(ColumnDef::new("cake_id").integer().not_null())
        .col(ColumnDef::new("baker_id").integer().not_null())
        .primary_key(
            Index::create()
                .name("cakes_bakers_pkey")
                .col("cake_id")
                .col("baker_id"),
        )
        .to_owned()
}

fn create_cake_table() -> TableCreateStatement {
    Table::create()
        .table("cake")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("name").string())
        .col(ColumnDef::new("price").decimal_len(19, 4))
        .col(ColumnDef::new("bakery_id").integer().not_null())
        .col(ColumnDef::new("gluten_free").boolean())
        .col(ColumnDef::new("serial").uuid())
        .primary_key(Index::create().primary().name("cake_pkey").col("id"))
        .foreign_key(
            ForeignKey::create()
                .name("FK_cake_bakery")
                .from("cake", "bakery_id")
                .to("bakery", "id")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_collection_table() -> TableCreateStatement {
    Table::create()
        .table("collection")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(
            ColumnDef::new("integers")
                .array(ColumnType::Integer)
                .not_null(),
        )
        .col(ColumnDef::new("integers_opt").array(ColumnType::Integer))
        .col(
            ColumnDef::new("case_insensitive_text")
                .custom("citext")
                .not_null(),
        )
        .to_owned()
}

fn create_parent_table() -> TableCreateStatement {
    Table::create()
        .table("parent")
        .col(ColumnDef::new("id1").integer().not_null())
        .col(ColumnDef::new("id2").integer().not_null())
        .primary_key(
            Index::create()
                .primary()
                .name("parent_pkey")
                .col("id1")
                .col("id2"),
        )
        .to_owned()
}

fn create_child_table() -> TableCreateStatement {
    Table::create()
        .table("child")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("parent_id1").integer().not_null())
        .col(ColumnDef::new("parent_id2").integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("FK_child_parent")
                .from("child", ("parent_id1", "parent_id2"))
                .to("parent", ("id1", "id2"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_db_types_table() -> TableCreateStatement {
    Table::create()
        .table("db_types")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("binary_1").binary())
        .col(ColumnDef::new("binary_2").binary_len(1))
        .col(ColumnDef::new("binary_3").binary_len(16))
        .col(ColumnDef::new("var_binary_1").var_binary(1))
        .col(ColumnDef::new("var_binary_2").var_binary(16))
        .col(ColumnDef::new("var_binary_3").var_binary(32))
        .col(ColumnDef::new("bit_1").bit(Some(1)))
        .col(ColumnDef::new("bit_2").bit(Some(16)))
        .col(ColumnDef::new("bit_3").bit(Some(32)))
        .col(ColumnDef::new("var_bit_1").varbit(1))
        .col(ColumnDef::new("var_bit_2").varbit(16))
        .col(ColumnDef::new("var_bit_3").varbit(32))
        .col(ColumnDef::new("bool").boolean())
        .primary_key(Index::create().primary().name("db_types_pkey").col("id"))
        .to_owned()
}

fn create_fkey_parent_table() -> TableCreateStatement {
    Table::create()
        .table("fkey_parent_table")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("u").integer().not_null())
        .index(
            Index::create()
                .unique()
                .name("idx-fkey_parent_table-u")
                .col("u"),
        )
        .to_owned()
}

fn create_fkey_child_table() -> TableCreateStatement {
    Table::create()
        .table("fkey_child_table")
        .col(ColumnDef::new("fk_u").integer().not_null().auto_increment())
        .foreign_key(
            ForeignKey::create()
                .name("FK_tabl2_fkey_parent_table")
                .from("fkey_child_table", "fk_u")
                .to("fkey_parent_table", "u")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_fkey_parent_no_uniq_table() -> TableCreateStatement {
    Table::create()
        .table("fkey_parent_table_no_uniq")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("u").integer().not_null())
        .to_owned()
}

fn create_fkey_parent_no_uniq_table_rediscover() -> TableCreateStatement {
    Table::create()
        .table("fkey_parent_table_no_uniq")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("u").integer().not_null())
        .index(
            Index::create()
                .unique()
                .name("idx-fkey_parent_table_no_uniq-u")
                .col("u"),
        )
        .to_owned()
}

fn create_fkey_child_no_uniq_table() -> TableCreateStatement {
    Table::create()
        .table("fkey_child_table_no_uniq")
        .col(ColumnDef::new("fk_u").integer().not_null().auto_increment())
        .foreign_key(
            ForeignKey::create()
                .name("fkey-fkey_child_table_no_uniq")
                .from("fkey_child_table_no_uniq", "fk_u")
                .to("fkey_parent_table_no_uniq", "u")
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_fkey_pair_parent_table() -> TableCreateStatement {
    Table::create()
        .table("fkey_pair_parent")
        .col(ColumnDef::new("id").integer().not_null().auto_increment())
        .col(ColumnDef::new("left").integer().not_null())
        .col(ColumnDef::new("right").integer().not_null())
        .index(
            Index::create()
                .unique()
                .name("idx-fkey_pair_parent-unique")
                .col("left")
                .col("right"),
        )
        .to_owned()
}

fn create_fkey_pair_child_table() -> TableCreateStatement {
    Table::create()
        .table("fkey_pair_child")
        .col(ColumnDef::new("left").integer().not_null())
        .col(ColumnDef::new("right").integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("fkey-fkey_pair_child-pair")
                .from("fkey_pair_child", ("left", "right"))
                .to("fkey_pair_parent", ("left", "right"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}
