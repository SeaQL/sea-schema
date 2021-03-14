use std::rc::Rc;
use async_std::task;
use sea_query::{Alias, Iden, MysqlQueryBuilder};
use sea_schema::mysql::query::SchemaQuery;
use sqlx::MySqlPool;

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

sea_schema::from_mysql_rows!();
use from_mysql_rows::*;

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
        let column = column_from_mysql_row(row);
        println!("{:?}", column);
    }
    println!();

}

// SELECT `column_name`, `column_type`, `is_nullable`, `column_key`, `column_default`, `extra`, `column_comment` FROM `information_schema`.`columns` WHERE `table_schema` = ? AND `table_name` = ? ORDER BY `ordinal_position` ASC
// ColumnQueryResult { column_name: "actor_id", column_type: "smallint unsigned", is_nullable: "NO", column_key: "PRI", column_default: None, extra: "auto_increment", column_comment: "" }
// ColumnQueryResult { column_name: "first_name", column_type: "varchar(45)", is_nullable: "NO", column_key: "", column_default: None, extra: "", column_comment: "" }
// ColumnQueryResult { column_name: "last_name", column_type: "varchar(45)", is_nullable: "NO", column_key: "MUL", column_default: None, extra: "", column_comment: "" }
// ColumnQueryResult { column_name: "last_update", column_type: "timestamp", is_nullable: "NO", column_key: "", column_default: Some("CURRENT_TIMESTAMP"), extra: "DEFAULT_GENERATED on update CURRENT_TIMESTAMP", column_comment: "" }