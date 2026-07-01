use crate::postgres::{def::*, query::ForeignKeyQueryResult};

/// Assumed to be ordered by constraint name, then the foreign key column's
/// ordinal position, so rows of one foreign key are consecutive.
pub fn parse_foreign_key_query_results(results: Vec<ForeignKeyQueryResult>) -> Vec<References> {
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
        let table = result.foreign_table_name;
        let on_update = parse_referential_action(result.on_update);
        let on_delete = parse_referential_action(result.on_delete);
        let mut columns = vec![result.column_name];
        let mut foreign_columns = vec![result.foreign_column_name];

        for result in results.by_ref() {
            if result.constraint_name != constraint_name {
                curr = Some(result);
                break;
            }

            columns.push(result.column_name);
            foreign_columns.push(result.foreign_column_name);
        }

        output.push(References {
            name: constraint_name,
            columns,
            table,
            foreign_columns,
            on_update,
            on_delete,
        });
    }

    output
}

fn parse_referential_action(code: Option<String>) -> Option<ForeignKeyAction> {
    match code.as_deref() {
        Some("a") => Some(ForeignKeyAction::NoAction),
        Some("r") => Some(ForeignKeyAction::Restrict),
        Some("c") => Some(ForeignKeyAction::Cascade),
        Some("n") => Some(ForeignKeyAction::SetNull),
        Some("d") => Some(ForeignKeyAction::SetDefault),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn foreign_key_row(
        constraint: &str,
        column: &str,
        foreign_table: &str,
        foreign_column: &str,
    ) -> ForeignKeyQueryResult {
        ForeignKeyQueryResult {
            constraint_name: constraint.to_owned(),
            column_name: column.to_owned(),
            foreign_table_name: foreign_table.to_owned(),
            foreign_column_name: foreign_column.to_owned(),
            on_update: Some("a".to_owned()),
            on_delete: Some("c".to_owned()),
        }
    }

    /// Composite key: per-column rows are paired in order. `information_schema`
    /// discovery got this wrong (SeaQL/sea-orm#2662).
    #[test]
    fn builds_composite_foreign_key_with_correct_pairing() {
        let results = vec![
            foreign_key_row("fk_pair", "f_a", "parent", "a"),
            foreign_key_row("fk_pair", "f_b", "parent", "b"),
        ];

        assert_eq!(
            parse_foreign_key_query_results(results),
            vec![References {
                name: "fk_pair".to_owned(),
                columns: vec!["f_a".to_owned(), "f_b".to_owned()],
                table: "parent".to_owned(),
                foreign_columns: vec!["a".to_owned(), "b".to_owned()],
                on_update: Some(ForeignKeyAction::NoAction),
                on_delete: Some(ForeignKeyAction::Cascade),
            }]
        );
    }

    /// Rows for several foreign keys are grouped by constraint name into one
    /// [`References`] each.
    #[test]
    fn groups_rows_by_constraint_name() {
        let results = vec![
            foreign_key_row("fk_one", "a", "t1", "id"),
            foreign_key_row("fk_two", "b", "t2", "id"),
            foreign_key_row("fk_two", "c", "t2", "code"),
        ];

        let references = parse_foreign_key_query_results(results);

        assert_eq!(references.len(), 2);
        assert_eq!(references[0].name, "fk_one");
        assert_eq!(references[0].columns, vec!["a".to_owned()]);
        assert_eq!(references[0].foreign_columns, vec!["id".to_owned()]);
        assert_eq!(references[1].name, "fk_two");
        assert_eq!(references[1].columns, vec!["b".to_owned(), "c".to_owned()]);
        assert_eq!(
            references[1].foreign_columns,
            vec!["id".to_owned(), "code".to_owned()]
        );
    }
}
