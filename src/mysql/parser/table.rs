use crate::Name;
use crate::mysql::def::*;
use crate::mysql::query::TableQueryResult;

impl TableQueryResult {
    pub fn parse(self) -> TableInfo {
        parse_table_query_result(self)
    }
}

pub fn parse_table_query_result(result: TableQueryResult) -> TableInfo {
    TableInfo {
        name: result.table_name,
        engine: StorageEngine::from_str(result.engine.as_str()).unwrap(),
        auto_increment: result.auto_increment,
        char_set: result.table_char_set.as_deref().and_then(CharSet::from_str),
        collation: result
            .table_collation
            .as_deref()
            .and_then(Collation::from_str),
        comment: result.table_comment,
    }
}
