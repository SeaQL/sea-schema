use crate::{
    parser::Parser,
    postgres::{
        def::*,
        query::{
            SchemaQueryBuilder, TableConstraintsField, UniqueConstraintNameResult,
            UniqueQueryResult,
        },
    },
    Name,
};

// TODO: Parse UniqueConstraintNameResult into another SelectStatement
use sea_query::{Iden, SelectStatement};
use std::rc::Rc;

// FIXME: Better Err type
pub fn parse_unique_query_results(results: Vec<UniqueQueryResult>) -> Result<Unique, ()> {
    // Return an Err if not all `UniqueQueryResult`s in the Vec belong to the same constraint
    let mut constraint_name: Option<String> = None;
    let mut out = Unique(Vec::new());

    for result in results {
        if constraint_name.is_none() {
            constraint_name = Some(result.constraint_name);
        } else if constraint_name.as_ref().unwrap() != &result.constraint_name {
            return Err(());
        }

        out.0.push(result.column_name);
    }

    Ok(out)
}
