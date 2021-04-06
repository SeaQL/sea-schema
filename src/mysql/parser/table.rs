use crate::mysql::def::*;
use crate::mysql::query::TableQueryResult;

impl TableQueryResult {
    pub fn parse(self) -> SystemInfo {
        parse_table_query_result(self)
    }
}

// pub fn parse_table_query_result(result: TableQueryResult) -> TableInfo {
//     TableInfo {
//         name: result.table_name,
//         engine: ,
//         auto_increment: ,
//         collation: ,
//         comment: ,
//     }
// }
