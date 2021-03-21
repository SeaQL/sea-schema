use std::rc::Rc;
use async_std::task;
use sea_query::{Alias, Iden, MysqlQueryBuilder};
use sea_schema::mysql::query::{SchemaQuery, ColumnQueryResult, IndexQueryResult};
use sea_schema::mysql::parser::{parse_column_query_result, parse_index_query_results};
use sqlx::MySqlPool;

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

fn main() {

    let connection = task::block_on(async {
        MySqlPool::connect("mysql://sea:sea@localhost/sakila").await.unwrap()
    });
    let mut pool = connection.try_acquire().unwrap();

    let schema: Rc<dyn Iden> = Rc::new(Alias::new("sakila"));
    let table: Rc<dyn Iden> = Rc::new(Alias::new("film"));

    // Columns

    let (sql, values) = SchemaQuery::query_columns(schema.clone(), table.clone()).build(MysqlQueryBuilder);
    println!("{}", sql);
    println!();

    let rows = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });

    for row in rows.iter() {
        let result: ColumnQueryResult = row.into();
        println!("{:?}", result);
        let column = parse_column_query_result(result);
        println!("{:?}", column);
    }
    println!();

    // Indexes

    let (sql, values) = SchemaQuery::query_indexes(schema.clone(), table.clone()).build(MysqlQueryBuilder);
    println!("{}", sql);
    println!();

    let rows = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });

    let results: Vec<IndexQueryResult> = rows.iter().map(|row| {
        let result = row.into();
        println!("{:?}", result);
        return result;
    }).collect();
    println!();

    for index in parse_index_query_results(Box::new(results.into_iter())) {
        println!("{:?}", index);
    }
    println!();

}
