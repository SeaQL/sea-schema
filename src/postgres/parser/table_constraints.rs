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

                    if let (Some(col), Some(ref_col)) =
                        (result.column_name, result.referential_key_column_name)
                    {
                        columns.push(col);
                        foreign_columns.push(ref_col);
                    }
                }

                dedup_preserving_order(&mut columns);
                dedup_preserving_order(&mut foreign_columns);

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

/// Remove duplicated values in-place while preserving first-seen order.
///
/// A well-formed key never repeats a column, so this is a no-op for valid
/// input. It only repairs the degenerate case where schema discovery cannot
/// correlate a composite foreign key's columns with the referenced columns:
/// when the key references a bare `UNIQUE INDEX` rather than a named `UNIQUE` /
/// `PRIMARY KEY` constraint, Postgres' `information_schema` does not expose the
/// referenced constraint, so `query_table_constraints` falls back to the
/// cartesian product of the local and referenced column lists. `Vec::dedup`
/// would only collapse *consecutive* duplicates, leaving interleaved
/// repetitions such as `[a, b, a, b]` untouched.
fn dedup_preserving_order(columns: &mut Vec<String>) {
    let mut seen = std::collections::HashSet::new();
    columns.retain(|column| seen.insert(column.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn foreign_key_row(column: &str, referenced: &str) -> TableConstraintsQueryResult {
        TableConstraintsQueryResult {
            constraint_name: "fk_pair".to_owned(),
            constraint_type: "FOREIGN KEY".to_owned(),
            column_name: Some(column.to_owned()),
            referential_key_table_name: Some("parent".to_owned()),
            referential_key_column_name: Some(referenced.to_owned()),
            update_rule: Some("NO ACTION".to_owned()),
            delete_rule: Some("CASCADE".to_owned()),
            ..Default::default()
        }
    }

    /// When a composite foreign key references a bare `UNIQUE INDEX` rather than
    /// a named constraint, `information_schema` cannot correlate the local and
    /// referenced columns, so the discovery query returns their cartesian
    /// product. It is ordered by the local column's ordinal position only,
    /// which interleaves the referenced columns as `[left, right, left, right]`.
    /// The parser must collapse this back to the intended 1:1 column pairing
    /// rather than emitting duplicated columns. See SeaQL/sea-orm#2662.
    #[test]
    fn dedups_cartesian_product_for_composite_foreign_key() {
        let results = vec![
            foreign_key_row("left", "left"),
            foreign_key_row("left", "right"),
            foreign_key_row("right", "left"),
            foreign_key_row("right", "right"),
        ];

        assert_eq!(
            parse_table_constraint_query_results(results),
            vec![Constraint::References(References {
                name: "fk_pair".to_owned(),
                columns: vec!["left".to_owned(), "right".to_owned()],
                table: "parent".to_owned(),
                foreign_columns: vec!["left".to_owned(), "right".to_owned()],
                on_update: Some(ForeignKeyAction::NoAction),
                on_delete: Some(ForeignKeyAction::Cascade),
            })]
        );
    }

    /// A well-formed composite foreign key (no duplicated columns) is preserved
    /// verbatim, including the local-to-referenced column pairing.
    #[test]
    fn keeps_well_formed_composite_foreign_key() {
        let results = vec![foreign_key_row("f_a", "a"), foreign_key_row("f_b", "b")];

        assert_eq!(
            parse_table_constraint_query_results(results),
            vec![Constraint::References(References {
                name: "fk_pair".to_owned(),
                columns: vec!["f_a".to_owned(), "f_b".to_owned()],
                table: "parent".to_owned(),
                foreign_columns: vec!["a".to_owned(), "b".to_owned()],
                on_update: Some(ForeignKeyAction::NoAction),
                on_delete: Some(ForeignKeyAction::Cascade),
            })]
        );
    }
}
