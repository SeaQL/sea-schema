#[derive(Debug, sea_query::Iden)]
pub enum InformationSchema {
    #[iden = "information_schema"]
    Schema,
    Columns,
}
