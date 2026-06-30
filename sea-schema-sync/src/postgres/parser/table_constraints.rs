use crate::postgres::{def::*, query::TableConstraintsQueryResult};

/// Assumed to be ordered by constraint name, then ordinal position.
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
