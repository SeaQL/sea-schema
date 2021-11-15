use sea_schema::postgres::def::{EnumDef, PgEnum, StringAttr, Type};
use sqlx::PgPool;

#[async_std::main]
async fn main() {
    let connection = PgPool::connect("postgres://sea:sea@localhost/sakila")
        .await
        .unwrap();

    let rows: Vec<PgEnum> = sqlx::query_as("SELECT pg_type.typname, pg_enum.enumlabel FROM pg_type JOIN pg_enum ON pg_enum.enumtypid = pg_type.oid;",
    )
    .fetch_all(&connection)
    .await
    .unwrap();

    let custom_enum = PgEnum::new().to_enum_type(rows);

    let font_family = r#"CREATE TYPE "font_family" AS ENUM ('serif', 'sans', 'monospace')"#;

    let rating = r#"CREATE TYPE "mpaa_rating" AS ENUM ('G', 'PG', 'PG-13', 'R', 'NC-17')"#;

    let parse_font_family = Type::enum_from_query(font_family);
    let parse_rating = Type::enum_from_query(rating);

    assert_eq!(
        parse_font_family,
        Type::Enum(EnumDef {
            values: vec![
                "serif".to_owned(),
                "sans".to_owned(),
                "monospace".to_owned(),
            ],
            attr: StringAttr { length: Some(11,) },
            typename: "font_family".to_owned(),
        },)
    );

    // Check if the end of parsed statement is same as the sql query
    assert_eq!(
        match parse_font_family {
            Type::Enum(enum_def) => Some(enum_def.to_sql_query()),
            _ => None,
        },
        Some(font_family.to_owned())
    );

    assert_eq!(
        parse_rating,
        Type::Enum(EnumDef {
            values: vec![
                "G".to_owned(),
                "PG".to_owned(),
                "PG-13".to_owned(),
                "R".to_owned(),
                "NC-17".to_owned(),
            ],
            attr: StringAttr { length: Some(11,) },
            typename: "mpaa_rating".to_owned(),
        },)
    );

    // Check if the end of parsed statement is same as the sql query
    assert_eq!(
        match parse_rating {
            Type::Enum(enum_def) => Some(enum_def.to_sql_query()),
            _ => None,
        },
        Some(rating.to_owned())
    );

    assert_eq!(
        custom_enum.enum_to_create_statement(),
        Some(r#"CREATE TYPE "mpaa_rating" AS ENUM ('G', 'PG', 'PG-13', 'R', 'NC-17')"#.to_owned())
    );

    assert_eq!(
        custom_enum,
        Type::Enum(EnumDef {
            values: vec![
                "G".to_owned(),
                "PG".to_owned(),
                "PG-13".to_owned(),
                "R".to_owned(),
                "NC-17".to_owned(),
            ],
            attr: StringAttr { length: Some(11,) },
            typename: "mpaa_rating".to_owned(),
        },)
    );
}
