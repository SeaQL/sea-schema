use crate::postgres::def::*;
use crate::postgres::query::TableQueryResult;

impl TableQueryResult {
    pub fn parse(self) -> TableInfo {
        parse_table_query_result(self)
    }
}

pub fn parse_table_query_result(table_query: TableQueryResult) -> TableInfo {
    let of_type = if let Some(type_name) = table_query.user_defined_type_name {
        Some(Type::from_str(&type_name))
    } else {
        None
    };

    TableInfo {
        name: table_query.table_name,
        of_type,
    }
}
