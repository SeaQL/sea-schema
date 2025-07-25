use pretty_assertions::assert_eq;
use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sea_schema::sea_query::TableRef;
use sea_schema::sea_query::{
    Alias, ColumnDef, ColumnType, Expr, ForeignKey, ForeignKeyAction, Index, PostgresQueryBuilder,
    Table, TableCreateStatement, extension::postgres::Type,
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
        .unwrap_or_else(|_| "postgres://root:root@localhost".to_owned());

    let connection = setup(&url, "sea-schema").await;
    let mut executor = connection.acquire().await.unwrap();

    sqlx::query("CREATE EXTENSION IF NOT EXISTS citext")
        .execute(&mut *executor)
        .await
        .unwrap();

    let create_enum_stmt = Type::create()
        .as_enum(Alias::new("crazy_enum"))
        .values(vec![
            Alias::new("Astro0%00%8987,.!@#$%^&*()_-+=[]{}\\|.<>/? ``"),
            Alias::new("Biology"),
            Alias::new("Chemistry"),
            Alias::new("Math"),
            Alias::new("Physics"),
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
    ];

    for tbl_create_stmt in tbl_create_stmts.iter() {
        let sql = tbl_create_stmt.to_string(PostgresQueryBuilder);
        println!("{sql};");
        println!();
        sqlx::query(&sql).execute(&mut *executor).await.unwrap();
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
            Some(TableRef::Table(tbl)) => tbl.to_string(),
            _ => unimplemented!(),
        };
        let table = map.get(&tbl_name).unwrap();
        let sql = table.write().to_string(PostgresQueryBuilder);
        println!("Expected SQL:");
        println!("{expected_sql};");
        println!("Generated SQL:");
        println!("{sql};");
        println!();
        assert_eq!(expected_sql, sql);
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
        .table(Alias::new("bakery"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("profit_margin")).double())
        .col(ColumnDef::new(Alias::new("crazy_enum_col")).custom(Alias::new("crazy_enum")))
        .primary_key(
            Index::create()
                .primary()
                .name("bakery_pkey")
                .col(Alias::new("id")),
        )
        .to_owned()
}

fn create_baker_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("baker"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("contact_details")).json())
        .col(ColumnDef::new(Alias::new("bakery_id")).integer())
        .primary_key(
            Index::create()
                .primary()
                .name("baker_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_baker_bakery")
                .from(Alias::new("baker"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_customer_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("customer"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("notes")).text())
        .primary_key(
            Index::create()
                .primary()
                .name("customer_pkey")
                .col(Alias::new("id")),
        )
        .to_owned()
}

fn create_order_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("order"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("total")).decimal_len(19, 4))
        .col(ColumnDef::new(Alias::new("bakery_id")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("customer_id"))
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("placed_at"))
                .date_time()
                .not_null()
                .default(Expr::current_timestamp()),
        )
        .col(
            ColumnDef::new(Alias::new("updated"))
                .date_time()
                .not_null()
                .extra("DEFAULT '2023-06-07 16:24:00'::timestamp without time zone"),
        )
        .col(
            ColumnDef::new(Alias::new("net_weight"))
                .double()
                .not_null()
                .default(10.05),
        )
        .col(
            ColumnDef::new(Alias::new("priority"))
                .integer()
                .not_null()
                .default(5),
        )
        .primary_key(
            Index::create()
                .primary()
                .name("order_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_order_bakery")
                .from(Alias::new("order"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_order_customer")
                .from(Alias::new("order"), Alias::new("customer_id"))
                .to(Alias::new("customer"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_lineitem_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("lineitem"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("price")).decimal_len(19, 4))
        .col(ColumnDef::new(Alias::new("quantity")).integer())
        .col(ColumnDef::new(Alias::new("order_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .primary_key(
            Index::create()
                .primary()
                .name("lineitem_pkey")
                .col(Alias::new("id")),
        )
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id")
                .col(Alias::new("cake_id")),
        )
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id_order_id")
                .col(Alias::new("cake_id"))
                .col(Alias::new("order_id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_lineitem_cake")
                .from(Alias::new("lineitem"), Alias::new("cake_id"))
                .to(Alias::new("cake"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_lineitem_order")
                .from(Alias::new("lineitem"), Alias::new("order_id"))
                .to(Alias::new("order"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_cakes_bakers_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("cakes_bakers"))
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("baker_id")).integer().not_null())
        .primary_key(
            Index::create()
                .name("cakes_bakers_pkey")
                .col(Alias::new("cake_id"))
                .col(Alias::new("baker_id")),
        )
        .to_owned()
}

fn create_cake_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("cake"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("name")).string())
        .col(ColumnDef::new(Alias::new("price")).decimal_len(19, 4))
        .col(ColumnDef::new(Alias::new("bakery_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("gluten_free")).boolean())
        .col(ColumnDef::new(Alias::new("serial")).uuid())
        .primary_key(
            Index::create()
                .primary()
                .name("cake_pkey")
                .col(Alias::new("id")),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_cake_bakery")
                .from(Alias::new("cake"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_collection_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("collection"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(Alias::new("integers"))
                .array(ColumnType::Integer)
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("integers_opt")).array(ColumnType::Integer))
        .col(
            ColumnDef::new(Alias::new("case_insensitive_text"))
                .custom(Alias::new("citext"))
                .not_null(),
        )
        .to_owned()
}

fn create_parent_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("parent"))
        .col(ColumnDef::new(Alias::new("id1")).integer().not_null())
        .col(ColumnDef::new(Alias::new("id2")).integer().not_null())
        .primary_key(
            Index::create()
                .primary()
                .name("parent_pkey")
                .col(Alias::new("id1"))
                .col(Alias::new("id2")),
        )
        .to_owned()
}

fn create_child_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("child"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(Alias::new("parent_id1"))
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(Alias::new("parent_id2"))
                .integer()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_child_parent")
                .from(
                    Alias::new("child"),
                    (Alias::new("parent_id1"), Alias::new("parent_id2")),
                )
                .to(Alias::new("parent"), (Alias::new("id1"), Alias::new("id2")))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_db_types_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("db_types"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("binary_1")).binary())
        .col(ColumnDef::new(Alias::new("binary_2")).binary_len(1))
        .col(ColumnDef::new(Alias::new("binary_3")).binary_len(16))
        .col(ColumnDef::new(Alias::new("var_binary_1")).var_binary(1))
        .col(ColumnDef::new(Alias::new("var_binary_2")).var_binary(16))
        .col(ColumnDef::new(Alias::new("var_binary_3")).var_binary(32))
        .col(ColumnDef::new(Alias::new("bit_1")).bit(Some(1)))
        .col(ColumnDef::new(Alias::new("bit_2")).bit(Some(16)))
        .col(ColumnDef::new(Alias::new("bit_3")).bit(Some(32)))
        .col(ColumnDef::new(Alias::new("var_bit_1")).varbit(1))
        .col(ColumnDef::new(Alias::new("var_bit_2")).varbit(16))
        .col(ColumnDef::new(Alias::new("var_bit_3")).varbit(32))
        .col(ColumnDef::new(Alias::new("bool")).boolean())
        .primary_key(
            Index::create()
                .primary()
                .name("db_types_pkey")
                .col(Alias::new("id")),
        )
        .to_owned()
}

fn create_fkey_parent_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("fkey_parent_table"))
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .col(ColumnDef::new(Alias::new("u")).integer().not_null())
        .index(
            Index::create()
                .unique()
                .name("IDX_fkey_parent_table_unique_u")
                .col(Alias::new("u")),
        )
        .to_owned()
}

fn create_fkey_child_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("fkey_child_table"))
        .col(
            ColumnDef::new(Alias::new("fk_u"))
                .integer()
                .not_null()
                .auto_increment(),
        )
        .foreign_key(
            ForeignKey::create()
                .name("FK_tabl2_fkey_parent_table")
                .from(Alias::new("fkey_child_table"), Alias::new("fk_u"))
                .to(Alias::new("fkey_parent_table"), Alias::new("u"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}
