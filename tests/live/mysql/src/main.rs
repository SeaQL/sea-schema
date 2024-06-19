use pretty_assertions::assert_eq;
use regex::Regex;
use sea_schema::mysql::{def::TableDef, discovery::SchemaDiscovery};
use sea_schema::sea_query::{
    Alias, ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, MysqlQueryBuilder, Table,
    TableCreateStatement, TableRef,
};
use sqlx::{MySql, MySqlPool, Pool};
use std::collections::HashMap;

#[cfg_attr(test, async_std::test)]
#[cfg_attr(not(test), async_std::main)]
async fn main() {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .is_test(true)
    //     .init();

    let url = std::env::var("DATABASE_URL_LIVE")
        .unwrap_or_else(|_| "mysql://root:root@localhost".to_owned());

    let connection = setup(&url, "sea-schema").await;
    let mut executor = connection.acquire().await.unwrap();

    let tbl_create_stmts = vec![
        create_bakery_table(),
        create_baker_table(),
        create_customer_table(),
        create_order_table(),
        create_cake_table(),
        create_cakes_bakers_table(),
        create_lineitem_table(),
        create_parent_table(),
        create_child_table(),
        create_db_types_table(),
    ];

    for tbl_create_stmt in tbl_create_stmts.iter() {
        let sql = tbl_create_stmt.to_string(MysqlQueryBuilder);
        println!("{};", sql);
        println!();
        sqlx::query(&sql).execute(&mut *executor).await.unwrap();
    }

    let schema_discovery = SchemaDiscovery::new(connection, "sea-schema");

    let schema = schema_discovery
        .discover()
        .await
        .expect("Error discovering schema");

    println!("{:#?}", schema);

    let map: HashMap<String, TableDef> = schema
        .tables
        .iter()
        .map(|table| (table.info.name.clone(), table.clone()))
        .collect();

    for tbl_create_stmt in tbl_create_stmts.into_iter() {
        let expected_sql = tbl_create_stmt.to_string(MysqlQueryBuilder);
        let tbl_name = match tbl_create_stmt.get_table_name() {
            Some(TableRef::Table(tbl)) => tbl.to_string(),
            _ => unimplemented!(),
        };
        let table = map.get(&tbl_name).unwrap();
        let sql = table.write().to_string(MysqlQueryBuilder);
        let sql = strip_generated_sql(sql);
        println!("Expected SQL:");
        println!("{};", expected_sql);
        println!("Generated SQL:");
        println!("{};", sql);
        println!();
        assert_eq!(expected_sql, sql);
    }
}

fn strip_generated_sql(mut sql: String) -> String {
    for (pattern, replacement) in vec![
        (Regex::new(r"(?i) DEFAULT NULL").unwrap(), ""),
        (Regex::new(r"(?i)TINYINT\(\d+\)").unwrap(), "tinyint"),
        (Regex::new(r"(?i)SMALLINT\(\d+\)").unwrap(), "smallint"),
        (Regex::new(r"(?i)MEDIUMINT\(\d+\)").unwrap(), "mediumint"),
        (Regex::new(r"(?i)INT\(\d+\)").unwrap(), "int"),
        (Regex::new(r"(?i)BIGINT\(\d+\)").unwrap(), "bigint"),
        (Regex::new(r"(?i)LONGTEXT").unwrap(), "json"),
    ] {
        sql = pattern.replace_all(&sql, replacement).to_string();
    }
    sql
}

async fn setup(base_url: &str, db_name: &str) -> Pool<MySql> {
    let url = format!("{}/mysql", base_url);
    let mut connection = MySqlPool::connect(&url)
        .await
        .unwrap()
        .acquire()
        .await
        .unwrap();

    let _drop_db_result = sqlx::query(&format!("DROP DATABASE IF EXISTS `{}`;", db_name))
        .bind(db_name)
        .execute(&mut *connection)
        .await
        .unwrap();

    let _create_db_result = sqlx::query(&format!("CREATE DATABASE `{}`;", db_name))
        .execute(&mut *connection)
        .await
        .unwrap();

    let url = format!("{}/{}", base_url, db_name);
    MySqlPool::connect(&url).await.unwrap()
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
        .col(
            ColumnDef::new(Alias::new("profit_margin"))
                .double()
                .extra("UNSIGNED".to_owned()),
        )
        .primary_key(Index::create().col(Alias::new("id")))
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .index(
            Index::create()
                .name("FK_baker_bakery")
                .col(Alias::new("bakery_id")),
        )
        .primary_key(Index::create().col(Alias::new("id")))
        .foreign_key(
            ForeignKey::create()
                .name("FK_baker_bakery")
                .from(Alias::new("baker"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .primary_key(Index::create().col(Alias::new("id")))
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .col(
            ColumnDef::new(Alias::new("total"))
                .decimal_len(19, 4)
                .extra("UNSIGNED".to_owned()),
        )
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
                .default("2023-06-07 16:24:00"),
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
        .index(
            Index::create()
                .name("FK_order_bakery")
                .col(Alias::new("bakery_id")),
        )
        .index(
            Index::create()
                .name("FK_order_customer")
                .col(Alias::new("customer_id")),
        )
        .primary_key(Index::create().col(Alias::new("id")))
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
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .col(ColumnDef::new(Alias::new("quantity")).unsigned().not_null())
        .col(ColumnDef::new(Alias::new("order_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .index(
            Index::create()
                .name("FK_lineitem_cake")
                .col(Alias::new("cake_id")),
        )
        .index(
            Index::create()
                .name("FK_lineitem_order")
                .col(Alias::new("order_id")),
        )
        .primary_key(Index::create().col(Alias::new("id")))
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_cake_id")
                .col(Alias::new("cake_id")),
        )
        .index(
            Index::create()
                .unique()
                .name("UNI_lineitem_order_id_cake_id")
                .col(Alias::new("order_id"))
                .col(Alias::new("cake_id")),
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
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
        .to_owned()
}

fn create_cakes_bakers_table() -> TableCreateStatement {
    Table::create()
        .table(Alias::new("cakes_bakers"))
        .col(ColumnDef::new(Alias::new("cake_id")).integer().not_null())
        .col(ColumnDef::new(Alias::new("baker_id")).integer().not_null())
        .primary_key(
            Index::create()
                .col(Alias::new("cake_id"))
                .col(Alias::new("baker_id")),
        )
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .col(
            ColumnDef::new(Alias::new("gluten_free"))
                .tiny_unsigned()
                .not_null(),
        )
        .col(ColumnDef::new(Alias::new("serial")).uuid())
        .index(
            Index::create()
                .name("FK_cake_bakery")
                .col(Alias::new("bakery_id")),
        )
        .primary_key(Index::create().col(Alias::new("id")))
        .foreign_key(
            ForeignKey::create()
                .name("FK_cake_bakery")
                .from(Alias::new("cake"), Alias::new("bakery_id"))
                .to(Alias::new("bakery"), Alias::new("id"))
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
                .col(Alias::new("id1"))
                .col(Alias::new("id2")),
        )
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .index(
            Index::create()
                .name("FK_child_parent")
                .col(Alias::new("parent_id1"))
                .col(Alias::new("parent_id2")),
        )
        .primary_key(Index::create().col(Alias::new("id")))
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
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
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
        .col(ColumnDef::new(Alias::new("bit_1")).bit(Some(1)))
        .col(ColumnDef::new(Alias::new("bit_2")).bit(Some(16)))
        .col(ColumnDef::new(Alias::new("bit_3")).bit(Some(32)))
        .col(ColumnDef::new(Alias::new("year")).year())
        .primary_key(Index::create().col(Alias::new("id")))
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_general_ci")
        .to_owned()
}
