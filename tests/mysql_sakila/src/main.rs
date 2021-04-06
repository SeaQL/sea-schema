use std::rc::Rc;
use async_std::task;
use sea_query::{Alias, Iden, MysqlQueryBuilder};
use sea_schema::mysql::schema::SchemaDiscovery;
use sea_schema::mysql::query::{SchemaQueryBuilder, ColumnQueryResult, ConstraintQueryResult, IndexQueryResult, VersionQueryResult};
use sea_schema::mysql::parser::{parse_column_query_result, parse_index_query_results, parse_version_query_result, parse_constraint_query_results};
use sqlx::MySqlPool;

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::bind_query;

fn main() {

    let connection = task::block_on(async {
        MySqlPool::connect("mysql://sea:sea@localhost/sakila").await.unwrap()
    });

    let mut schema = SchemaDiscovery::new(connection);

    task::block_on(async {
        schema.discover().await
    });

    panic!("bye");

    // let schema: Rc<dyn Iden> = Rc::new(Alias::new("sakila"));
    // let table: Rc<dyn Iden> = Rc::new(Alias::new("film"));

    // // Columns

    // let (sql, values) = schema_query.query_columns(schema.clone(), table.clone()).build(MysqlQueryBuilder);
    // println!("{}", sql);
    // println!();

    // let rows = task::block_on(async {
    //     bind_query(sqlx::query(&sql), &values)
    //         .fetch_all(&mut pool)
    //         .await
    //         .unwrap()
    // });

    // for row in rows.iter() {
    //     let result: ColumnQueryResult = row.into();
    //     println!("{:?}", result);
    //     let column = parse_column_query_result(result);
    //     println!("{:?}", column);
    // }
    // println!();

    // // Indexes

    // let (sql, values) = schema_query.query_indexes(schema.clone(), table.clone()).build(MysqlQueryBuilder);
    // println!("{}", sql);
    // println!();

    // let rows = task::block_on(async {
    //     bind_query(sqlx::query(&sql), &values)
    //         .fetch_all(&mut pool)
    //         .await
    //         .unwrap()
    // });

    // let results: Vec<IndexQueryResult> = rows.iter().map(|row| {
    //     let result = row.into();
    //     println!("{:?}", result);
    //     return result;
    // }).collect();
    // println!();

    // for index in parse_index_query_results(Box::new(results.into_iter())) {
    //     println!("{:?}", index);
    // }
    // println!();

    // // Foreign Key

    // let (sql, values) = schema_query.query_constraints(schema.clone(), table.clone()).build(MysqlQueryBuilder);
    // println!("{}", sql);
    // println!();

    // let rows = task::block_on(async {
    //     bind_query(sqlx::query(&sql), &values)
    //         .fetch_all(&mut pool)
    //         .await
    //         .unwrap()
    // });

    // let results: Vec<ConstraintQueryResult> = rows.iter().map(|row| {
    //     let result = row.into();
    //     println!("{:?}", result);
    //     return result;
    // }).collect();
    // println!();

    // for index in parse_constraint_query_results(Box::new(results.into_iter())) {
    //     println!("{:?}", index);
    // }
    // println!();

}
