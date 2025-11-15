use crate::Name;
use crate::postgres::{def::*, query::TableConstraintsQueryResult};

/// Assumed to be ordered by table name, then constraint name, then ordinal position, then the
/// constraint name of the foreign key, then the ordinal position of the foreign key
pub fn parse_table_constraint_query_results(
    results: Vec<TableConstraintsQueryResult>,
) -> Vec<Constraint> {
    let mut output = Vec::new();
    let mut results = results.into_iter();
    let mut curr = None;

    loop {
        let result = if let Some(result) = curr.take() {
            result
        } else if let Some(result) = results.next() {
            result
        } else {
            break;
        };

        let constraint_name = result.constraint_name;
        match result.constraint_type.as_str() {
            "CHECK" => {
                if let Some(check_clause) = result.check_clause {
                    output.push(Constraint::Check(Check {
                        name: constraint_name,
                        expr: check_clause,
                        // TODO: How to find?
                        no_inherit: false,
                    }));
                }
            }

            "FOREIGN KEY" => {
                let mut columns = Vec::new();
                let mut foreign_columns = Vec::new();

                columns.push(result.column_name.unwrap());
                let Some(table) = result.referential_key_table_name else {
                    continue;
                };
                foreign_columns.push(result.referential_key_column_name.unwrap());
                let on_update =
                    ForeignKeyAction::from_str(&result.update_rule.clone().unwrap_or_default());
                let on_delete =
                    ForeignKeyAction::from_str(&result.delete_rule.clone().unwrap_or_default());

                for result in results.by_ref() {
                    if result.constraint_name != constraint_name {
                        curr = Some(result);
                        break;
                    }

                    if result.column_name.is_some() && result.referential_key_column_name.is_some()
                    {
                        columns.push(result.column_name.unwrap());
                        foreign_columns.push(result.referential_key_column_name.unwrap());
                    }
                }

                columns.dedup();
                foreign_columns.dedup();

                output.push(Constraint::References(References {
                    name: constraint_name,
                    columns,
                    table,
                    foreign_columns,
                    on_update,
                    on_delete,
                }));
            }

            "PRIMARY KEY" => {
                let mut columns = vec![result.column_name.unwrap()];

                for result in results.by_ref() {
                    if result.constraint_name != constraint_name {
                        curr = Some(result);
                        break;
                    }

                    columns.push(result.column_name.unwrap());
                }

                output.push(Constraint::PrimaryKey(PrimaryKey {
                    name: constraint_name,
                    columns,
                }));
            }

            "UNIQUE" => {
                let mut columns = vec![result.column_name.unwrap()];

                for result in results.by_ref() {
                    if result.constraint_name != constraint_name {
                        curr = Some(result);
                        break;
                    }

                    columns.push(result.column_name.unwrap());
                }

                output.push(Constraint::Unique(Unique {
                    name: constraint_name,
                    columns,
                    is_partial: false,
                }));
            }

            _ => {
                // FIXME: Invalid input error handling
            }
        }
    }

    output
}
