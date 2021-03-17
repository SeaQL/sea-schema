use std::rc::Rc;
use async_std::task;
use sea_query::{Alias, Iden, MysqlQueryBuilder};
use sea_schema::mysql::query::{SchemaQuery, ColumnQueryResult};
use sea_schema::mysql::parser::parse_column_query_result;
use sqlx::MySqlPool;

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

fn main() {

    let connection = task::block_on(async {
        MySqlPool::connect("mysql://sea:sea@localhost/sakila").await.unwrap()
    });
    let mut pool = connection.try_acquire().unwrap();

    let schema: Rc<dyn Iden> = Rc::new(Alias::new("sakila"));
    let table: Rc<dyn Iden> = Rc::new(Alias::new("actor"));
    let (sql, values) = SchemaQuery::query_columns(schema.clone(), table.clone()).build(MysqlQueryBuilder);
    println!("{}", sql);

    let rows = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });

    for row in rows.iter() {
        let column: ColumnQueryResult = row.into();
        let schema = parse_column_query_result(column);
        println!("{:?}", schema);
    }
    println!();

}

// SELECT `column_name`, `column_type`, `is_nullable`, `column_key`, `column_default`, `extra`, `column_comment` FROM `information_schema`.`columns` WHERE `table_schema` = ? AND `table_name` = ? ORDER BY `ordinal_position` ASC

// ColumnQueryResult { column_name: "actor_id", column_type: "smallint unsigned", is_nullable: "NO", column_key: "PRI", column_default: None, extra: "auto_increment", column_comment: "" }
// ColumnQueryResult { column_name: "first_name", column_type: "varchar(45)", is_nullable: "NO", column_key: "", column_default: None, extra: "", column_comment: "" }
// ColumnQueryResult { column_name: "last_name", column_type: "varchar(45)", is_nullable: "NO", column_key: "MUL", column_default: None, extra: "", column_comment: "" }
// ColumnQueryResult { column_name: "last_update", column_type: "timestamp", is_nullable: "NO", column_key: "", column_default: Some("CURRENT_TIMESTAMP"), extra: "DEFAULT_GENERATED on update CURRENT_TIMESTAMP", column_comment: "" }

// ColumnInfo { name: "actor_id", col_type: SmallInt(NumericAttr { maximum: None, decimal: None, unsigned: Some(true), zero_fill: None }), key: Primary, default: None, extra: ColumnExtra { auto_increment: true, on_update_current_timestamp: false, generated: false, default_generated: false }, comment: "" }
// ColumnInfo { name: "first_name", col_type: Varchar(StringAttr { length: Some(45), charset_name: None, collation_name: None }), key: Null, default: None, extra: ColumnExtra { auto_increment: false, on_update_current_timestamp: false, generated: false, default_generated: false }, comment: "" }
// ColumnInfo { name: "last_name", col_type: Varchar(StringAttr { length: Some(45), charset_name: None, collation_name: None }), key: Multiple, default: None, extra: ColumnExtra { auto_increment: false, on_update_current_timestamp: false, generated: false, default_generated: false }, comment: "" }
// ColumnInfo { name: "last_update", col_type: Timestamp(TimeAttr { fractional: None }), key: Null, default: Some(ColumnDefault { expr: "CURRENT_TIMESTAMP" }), extra: ColumnExtra { auto_increment: false, on_update_current_timestamp: true, generated: false, default_generated: true }, comment: "" }
