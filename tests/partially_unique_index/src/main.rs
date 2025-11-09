use postgresql_embedded::blocking::PostgreSQL;

use sea_schema::postgres::discovery::SchemaDiscovery;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup()?;
    postgresql.start()?;

    let database_name = "test_partially_unique";
    postgresql.create_database(database_name)?;

    let database_url = postgresql.settings();

    let conn = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url.url(database_name))
        .await?;

    sqlx::query(
        "
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) NOT NULL,
            username VARCHAR(100),
            is_active BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        ",
    )
    .execute(&conn)
    .await?;

    sqlx::query(
        "
        CREATE UNIQUE INDEX idx_users_email_active
        ON users (email)
        WHERE is_active = true;
        ",
    )
    .execute(&conn)
    .await?;

    sqlx::query(
        "
        CREATE UNIQUE INDEX uniq_users_username ON users (username);
        ",
    )
    .execute(&conn)
    .await?;

    let schema_discovery = SchemaDiscovery::new(conn, "public");

    let schema = schema_discovery.discover().await?;

    let tb = schema
        .tables
        .iter()
        .find(|t| t.info.name == "users")
        .unwrap();
    let is_partial = {
        tb.unique_constraints
            .iter()
            .find(|uc| uc.name == "idx_users_email_active")
            .unwrap()
            .is_partial
    };
    assert!(is_partial);

    let is_not_partial = {
        tb.unique_constraints
            .iter()
            .find(|uc| uc.name == "uniq_users_username")
            .unwrap()
            .is_partial
    };

    assert!(!is_not_partial);

    postgresql.stop()?;

    Ok(())
}
