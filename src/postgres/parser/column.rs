use crate::{
    parser::Parser,
    postgres::{
        def::*,
        parser::yes_or_no_to_bool,
        query::{constraints, ColumnQueryResult},
    },
    Name,
};

use std::convert::TryFrom;

impl ColumnQueryResult {
    pub fn parse(self) -> ColumnInfo {
        parse_column_query_result(self)
    }
}

pub fn parse_column_query_result(result: ColumnQueryResult) -> ColumnInfo {
    ColumnInfo {
        name: result.column_name.clone(),
        col_type: parse_column_type(&result),
        default: ColumnExpression::from_option_string(result.column_default),
        generated: ColumnExpression::from_option_string(result.column_generated),
        not_null: NotNull::from_bool(yes_or_no_to_bool(&result.is_nullable)),
    }
}

pub fn parse_column_type(result: &ColumnQueryResult) -> ColumnType {
    let mut parser_type = Parser::new(&result.column_type);

    if parser_type.curr().is_none() {
        return Type::Unknown(String::default());
    }

    let ctype = if let Some(word) = parser_type.next_if_unquoted_any() {
        Type::from_str(word.as_str())
    } else {
        Type::from_str("")
    };

    if ctype.has_numeric_attr() {
        parse_numeric_attributes(
            result.numeric_precision,
            result.numeric_precision_radix,
            result.numeric_scale,
            ctype,
        )
    } else {
        ctype
    }
}

pub fn parse_numeric_attributes(
    num_precision: Option<i32>,
    num_precision_radix: Option<i32>,
    num_scale: Option<i32>,
    mut ctype: ColumnType,
) -> ColumnType {
    let numeric_precision: Option<u16> = match num_precision {
        None => None,
        Some(num) => match u16::try_from(num) {
            Ok(num) => Some(num),
            Err(e) => None,
        },
    };
    let numeric_precision_radix: Option<u16> = match num_precision_radix {
        None => None,
        Some(num) => match u16::try_from(num) {
            Ok(num) => Some(num),
            Err(e) => None,
        },
    };
    let numeric_scale: Option<u16> = match num_scale {
        None => None,
        Some(num) => match u16::try_from(num) {
            Ok(num) => Some(num),
            Err(e) => None,
        },
    };

    match ctype {
        Type::Decimal(ref mut attr) | Type::Numeric(ref mut attr) => {
            attr.precision = numeric_precision;
            attr.scale = numeric_scale;
        }
        _ => panic!("parse_numeric_attributes(_) received a type other than Decimal or Numeric"),
    };

    ctype
}
