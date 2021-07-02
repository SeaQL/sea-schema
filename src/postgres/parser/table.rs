use crate::postgres::def::*;
use crate::postgres::query::{constraints::*, TableQueryResult};
use crate::Name;

impl TableQueryResult {
    pub fn parse(self) -> TableInfo {}
}

pub fn parse_table(
    table_query: TableQueryResult,
    table_constraints: Vec<TableConstraintQueryResult>,
    check_constraints: Vec<CheckConstraintQueryResult>,
    key_column_usage: Vec<KeyColumnUsageQueryResult>,
) -> TableInfo {

}
