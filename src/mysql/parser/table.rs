use crate::mysql::def::*;
use crate::mysql::query::TableQueryResult;
use crate::Name;

impl TableQueryResult {
    pub fn parse(self) -> TableInfo {
        parse_table_query_result(self)
    }
}

pub fn parse_table_query_result(result: TableQueryResult) -> TableInfo {
    let collation = parse_table_collation(result.table_collation.as_str());

    TableInfo {
        name: result.table_name,
        engine: parse_table_engine(result.engine.as_str()),
        auto_increment: result.auto_increment,
        char_set: collation.char_set(),
        collation,
        comment: result.table_comment,
    }
}

pub fn parse_table_engine(string: &str) -> StorageEngine {
    StorageEngine::from_str(string).unwrap()
}

pub fn parse_table_collation(string: &str) -> Collation {
    Collation::from_str(string).unwrap()
}