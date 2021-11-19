use sea_schema::postgres::{def::Type, discovery::SchemaDiscovery};
use sqlx::PgPool;

#[async_std::main]
async fn main() {
    let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let enums_discovery = SchemaDiscovery::new(connection, "public")
        .discover_enums()
        .await;

    // Assert that all enums in the database are equal to their
    // raw SQL queries
    enums_discovery.iter().for_each(|enum_type| {
        let to_sql_query = enum_type.get_enum_def().to_sql_query();
        assert_eq!(enum_type, &Type::enum_from_query(&to_sql_query));
    });
}
