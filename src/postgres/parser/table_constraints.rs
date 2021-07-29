use crate::postgres::{def::*, query::TableConstraintsQueryResult};
use crate::{parser::Parser, Name};
use sea_query::unescape_string;

pub struct TableConstraintsQueryResultParser {
    curr: Option<TableConstraintsQueryResult>,
    results: Box<dyn Iterator<Item = TableConstraintsQueryResult>>,
}

/// Assumed to be ordered by table name, then constraint name, then ordinal position, then the
/// constraint name of the foreign key, then the ordinal position of the foreign key
pub fn parse_table_constraint_query_results(
    results: Box<dyn Iterator<Item = TableConstraintsQueryResult>>,
) -> impl Iterator<Item = Constraint> {
    TableConstraintsQueryResultParser {
        curr: None,
        results,
    }
}

impl Iterator for TableConstraintsQueryResultParser {
    type Item = Constraint;

    // FIXME/TODO: How to handle invalid input
    fn next(&mut self) -> Option<Self::Item> {
        let result = if let Some(result) = self.curr.take() {
            result
        } else if let Some(result) = self.results.next() {
            result
        } else {
            return None;
        };

        let constraint_name = result.constraint_name;
        match result.constraint_type.as_str() {
            "CHECK" => {
                Some(Constraint::Check(Check {
                    expr: result.check_clause.unwrap().to_string(),
                    // TODO: How to find?
                    no_inherit: false,
                }))
            }

            "FOREIGN KEY" => {
                let mut columns = Vec::new();
                let mut foreign_columns = Vec::new();

                columns.push(result.column_name.unwrap());
                let table = result.referential_key_table_name.unwrap();
                foreign_columns.push(result.referential_key_column_name.unwrap());

                while let Some(result) = self.results.next() {
                    if result.constraint_name != constraint_name {
                        self.curr = Some(result);
                        return Some(Constraint::References(References {
                            columns,
                            table,
                            foreign_columns,
                        }));
                    }

                    columns.push(result.column_name.unwrap());
                    foreign_columns.push(result.referential_key_column_name.unwrap());
                }

                Some(Constraint::References(References {
                    columns,
                    table,
                    foreign_columns,
                }))
            }

            "PRIMARY KEY" => {
                let mut columns = Vec::new();

                columns.push(result.column_name.unwrap());

                while let Some(result) = self.results.next() {
                    if result.constraint_name != constraint_name {
                        self.curr = Some(result);
                        return Some(Constraint::PrimaryKey(PrimaryKey(columns)));
                    }

                    columns.push(result.column_name.unwrap());
                }

                Some(Constraint::PrimaryKey(PrimaryKey(columns)))
            }

            "UNIQUE" => {
                let mut columns = Vec::new();

                columns.push(result.column_name.unwrap());

                while let Some(result) = self.results.next() {
                    if result.constraint_name != constraint_name {
                        self.curr = Some(result);
                        return Some(Constraint::Unique(Unique(columns)));
                    }

                    columns.push(result.column_name.unwrap());
                }

                Some(Constraint::Unique(Unique(columns)))
            }

            _ => {
                // FIXME: Invalid input error handling
                None
            }
        }
    }
}
