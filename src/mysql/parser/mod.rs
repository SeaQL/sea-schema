use sea_query::{Token, Tokenizer};
use super::def::*;
use super::query::ColumnQueryResult;

pub fn parse_column_query_result(result: ColumnQueryResult) -> ColumnInfo {
    ColumnInfo {
        name: result.column_name,
        col_type: parse_column_type(result.column_type),
        key: parse_column_key(result.column_key),
        default: parse_column_default(result.column_default),
        extra: parse_column_extra(result.extra),
        comment: result.column_comment,
    }
}

pub fn parse_column_type(column_type: String) -> ColumnType {
    todo!()
}

pub fn parse_column_key(column_key: String) -> ColumnKey {
    match column_key.as_str() {
        "PRI" => ColumnKey::Primary,
        "UNI" => ColumnKey::Unique,
        "MUL" => ColumnKey::Multiple,
        _ => ColumnKey::Null,
    }
}

pub fn parse_column_default(column_default: Option<String>) -> Option<ColumnDefault> {
    match column_default {
        Some(default) => {
            if !default.is_empty() {
                Some(ColumnDefault {
                    expr: default
                })
            } else {
                None
            }
        },
        None => None,
    }
}

pub fn parse_column_extra(expr: String) -> ColumnExtra {
    let mut extra = ColumnExtra::default();
    let words: Vec<&str> = expr.split(" ").collect();

    let mut i = 0;
    while i < words.len() {
        let word = &words[i];
        match *word {
            "auto_increment" => { extra.auto_increment = true },
            "on" => {
                if i + 2 < words.len() && words[i + 1] == "update" && words[i + 2] == "CURRENT_TIMESTAMP" {
                    i += 2;
                    extra.on_update_current_timestamp = true;
                }
            },
            "STORED" | "VIRTUAL" => {
                if i + 1 < words.len() && words[i + 1] == "GENERATED" {
                    i += 1;
                    extra.generated = true;
                }
            },
            "DEFAULT_GENERATED" => {
                extra.default_generated = true;
            },
            _ => {},
        }
        i += 1;
    }
    extra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            parse_column_extra("DEFAULT_GENERATED on update CURRENT_TIMESTAMP".to_owned()),
            ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: true,
                generated: false,
                default_generated: true,
            }
        );
    }
}