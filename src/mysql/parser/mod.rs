use sea_query::{Token, Tokenizer, unescape_string};
use crate::parser::Parser;
use super::def::*;
use super::query::ColumnQueryResult;

pub fn parse_column_query_result(result: ColumnQueryResult) -> ColumnInfo {
    ColumnInfo {
        name: result.column_name,
        col_type: parse_column_type(into_tokens(&result.column_type)),
        key: parse_column_key(into_tokens(&result.column_key)),
        default: parse_column_default(result.column_default),
        extra: parse_column_extra(&mut Parser::new(&result.extra)),
        comment: result.column_comment,
    }
}

fn into_tokens(string: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new(string);
    let tokens: Vec<Token> = tokenizer.iter()
        .filter(|x| !x.is_space()) // retains non-space tokens
        .collect::<Vec<_>>().into_iter().rev().collect(); // reverse the vector so pop() is actually at front
    tokens
}

pub fn parse_column_type(mut tokens: Vec<Token>) -> ColumnType {
    let mut type_name = "";
    if tokens.is_empty() {
        return Type::Unknown(type_name.to_owned());
    }
    let tok = tokens.pop().unwrap();
    if tok.is_unquoted() {
        type_name = tok.as_str();
    }
    let ctype = parse_type_name(type_name);
    if ctype.is_numeric() {
        parse_numeric_attributes(tokens, ctype)
    } else if ctype.is_time() {
        parse_time_attributes(tokens, ctype)
    } else if ctype.is_string() {
        parse_string_attributes(tokens, ctype)
    } else if ctype.is_free_size_blob() {
        parse_blob_attributes(tokens, ctype)
    } else if ctype.is_enum() {
        parse_enum_definition(tokens, ctype)
    } else if ctype.is_set() {
        parse_set_definition(tokens, ctype)
    } else if ctype.is_geometry() {
        parse_geometry_attributes(tokens, ctype)
    } else {
        ctype
    }
}

pub fn parse_type_name(type_name: &str) -> Type {
    match type_name.to_lowercase().as_str() {
        "serial" => Type::Serial(NumericAttr::default()),
        "bit" => Type::Bit(NumericAttr::default()),
        "tinyint" => Type::TinyInt(NumericAttr::default()),
        "bool" => Type::Bool(NumericAttr::default()),
        "smallint" => Type::SmallInt(NumericAttr::default()),
        "mediumint" => Type::MediumInt(NumericAttr::default()),
        "int" => Type::Int(NumericAttr::default()),
        "bigint" => Type::BigInt(NumericAttr::default()),
        "decimal" => Type::Decimal(NumericAttr::default()),
        "float" => Type::Float(NumericAttr::default()),
        "double" => Type::Double(NumericAttr::default()),
        "date" => Type::Date,
        "time" => Type::Time(TimeAttr::default()),
        "datetime" => Type::DateTime(TimeAttr::default()),
        "timestamp" => Type::Timestamp(TimeAttr::default()),
        "year" => Type::Year,
        "char" => Type::Char(StringAttr::default()),
        "nchar" => Type::NChar(StringAttr::default()),
        "varchar" => Type::Varchar(StringAttr::default()),
        "nvarchar" => Type::NVarchar(StringAttr::default()),
        "binary" => Type::Binary(StringAttr::default()),
        "varbinary" => Type::Varbinary(StringAttr::default()),
        "text" => Type::Text(StringAttr::default()),
        "tinytext" => Type::TinyText(StringAttr::default()),
        "mediumtext" => Type::MediumText(StringAttr::default()),
        "longtext" => Type::LongText(StringAttr::default()),
        "blob" => Type::Blob(BlobAttr::default()),
        "tinyblob" => Type::TinyBlob,
        "mediumblob" => Type::MediumBlob,
        "longblob" => Type::LongBlob,
        "enum" => Type::Enum(EnumDef::default()),
        "set" => Type::Set(SetDef::default()),
        "geometry" => Type::Geometry(GeometryAttr::default()),
        "point" => Type::Point(GeometryAttr::default()),
        "linestring" => Type::LineString(GeometryAttr::default()),
        "polygon" => Type::Polygon(GeometryAttr::default()),
        "multipoint" => Type::MultiPoint(GeometryAttr::default()),
        "multilinestring" => Type::MultiLineString(GeometryAttr::default()),
        "multipolygon" => Type::MultiPolygon(GeometryAttr::default()),
        "geometrycollection" => Type::GeometryCollection(GeometryAttr::default()),
        "json" => Type::Json,
        _ => Type::Unknown(type_name.to_owned()),
    }
}

fn parse_numeric_attributes(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_punctuation() && tok.as_str() == "(" {
        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if tok.is_unquoted() && tok.as_str().parse::<u32>().is_ok() {
            ctype.get_numeric_attr_mut().maximum = Some(tok.as_str().parse::<u32>().unwrap());
        }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if tok.is_punctuation() && tok.as_str() == "," {
            if tokens.is_empty() { return ctype; }
            tok = tokens.pop().unwrap();

            if tok.is_unquoted() && tok.as_str().parse::<u32>().is_ok() {
                ctype.get_numeric_attr_mut().decimal = Some(tok.as_str().parse::<u32>().unwrap());
            }

            if tokens.is_empty() { return ctype; }
            tok = tokens.pop().unwrap();
        }

        if !(tok.is_punctuation() && tok.as_str() == ")") { return ctype; }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();
    }

    if tok.is_unquoted() && tok.as_str().to_lowercase() == "unsigned" {
        ctype.get_numeric_attr_mut().unsigned = Some(true);

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();
    }

    if tok.is_unquoted() && tok.as_str().to_lowercase() == "zerofill" {
        ctype.get_numeric_attr_mut().zero_fill = Some(true);
    }

    return ctype;
}

fn parse_time_attributes(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_punctuation() && tok.as_str() == "(" {
        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if tok.is_unquoted() && tok.as_str().parse::<u32>().is_ok() {
            ctype.get_time_attr_mut().fractional = Some(tok.as_str().parse::<u32>().unwrap());
        }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if !(tok.is_punctuation() && tok.as_str() == ")") { return ctype; }
    }

    return ctype;
}

fn parse_string_attributes(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_punctuation() && tok.as_str() == "(" {
        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if tok.is_unquoted() && tok.as_str().parse::<u32>().is_ok() {
            ctype.get_string_attr_mut().length = Some(tok.as_str().parse::<u32>().unwrap());
        }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if !(tok.is_punctuation() && tok.as_str() == ")") { return ctype; }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();
    }

    parse_charset_collate(tok, tokens, ctype.get_string_attr_mut());

    return ctype;
}

fn parse_charset_collate(mut tok: Token, mut tokens: Vec<Token>, str_attr: &mut StringAttr) {

    if tok.is_unquoted() && tok.as_str().to_lowercase() == "character" {
        if tokens.is_empty() { return; }
        tok = tokens.pop().unwrap();

        if tok.is_unquoted() && tok.as_str().to_lowercase() == "set" {
            if tokens.is_empty() { return; }
            tok = tokens.pop().unwrap();

            str_attr.charset_name = Some(tok.as_str().to_owned());

            if tokens.is_empty() { return; }
            tok = tokens.pop().unwrap();
        }
    }

    if tok.is_unquoted() && tok.as_str().to_lowercase() == "collate" {
        if tokens.is_empty() { return; }
        tok = tokens.pop().unwrap();

        str_attr.collation_name = Some(tok.as_str().to_owned());
    }
}

fn parse_blob_attributes(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_punctuation() && tok.as_str() == "(" {
        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if tok.is_unquoted() && tok.as_str().parse::<u32>().is_ok() {
            ctype.get_blob_attr_mut().length = Some(tok.as_str().parse::<u32>().unwrap());
        }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if !(tok.is_punctuation() && tok.as_str() == ")") { return ctype; }
    }

    return ctype;
}

fn parse_enum_definition(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_punctuation() && tok.as_str() == "(" {
        while !tokens.is_empty() {
            tok = tokens.pop().unwrap();
            if tok.is_quoted() {
                ctype.get_enum_def_mut().values.push(unescape_string(tok.unquote().unwrap().as_str()));
            }
        }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if !(tok.is_punctuation() && tok.as_str() == ")") { return ctype; }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();
    }

    parse_charset_collate(tok, tokens, &mut ctype.get_enum_def_mut().attr);

    return ctype;
}

fn parse_set_definition(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_punctuation() && tok.as_str() == "(" {
        while !tokens.is_empty() {
            tok = tokens.pop().unwrap();
            if tok.is_quoted() {
                ctype.get_set_def_mut().members.push(unescape_string(tok.unquote().unwrap().as_str()));
            }
        }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if !(tok.is_punctuation() && tok.as_str() == ")") { return ctype; }

        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();
    }

    parse_charset_collate(tok, tokens, &mut ctype.get_set_def_mut().attr);

    return ctype;
}

fn parse_geometry_attributes(mut tokens: Vec<Token>, mut ctype: ColumnType) -> ColumnType {
    if tokens.is_empty() { return ctype; }
    let mut tok = tokens.pop().unwrap();

    if tok.is_unquoted() && tok.as_str().to_lowercase() == "srid" {
        if tokens.is_empty() { return ctype; }
        tok = tokens.pop().unwrap();

        if tok.is_unquoted() && tok.as_str().parse::<u32>().is_ok() {
            ctype.get_geometry_attr_mut().srid = Some(tok.as_str().parse::<u32>().unwrap());
        }
    }

    return ctype;
}

pub fn parse_column_key(mut tokens: Vec<Token>) -> ColumnKey {
    if tokens.is_empty() { return ColumnKey::Null; }
    let tok = tokens.pop().unwrap();

    match tok.as_str() {
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

pub fn parse_column_extra(parser: &mut Parser) -> ColumnExtra {
    let mut extra = ColumnExtra::default();

    if let Some(tok) = parser.curr() {
        if tok.is_unquoted() && tok.as_str().to_lowercase() == "auto_increment" {
            parser.next();

            extra.auto_increment = true;
        }
    }

    if let Some(tok) = parser.curr() {
        if tok.is_unquoted() && tok.as_str().to_lowercase() == "default_generated" {
            parser.next();

            extra.default_generated = true;
        }
    }

    if let Some(tok) = parser.curr() {
        if tok.is_unquoted() && tok.as_str().to_lowercase() == "on" {
            parser.next();

            if let Some(tok) = parser.curr() {
                if tok.is_unquoted() && tok.as_str().to_lowercase() == "update" {
                    parser.next();

                    if let Some(tok) = parser.curr() {
                        if tok.is_unquoted() && tok.as_str().to_lowercase() == "current_timestamp" {
                            parser.next();

                            extra.on_update_current_timestamp = true;
                        }
                    }
                }
            }
        }
    }

    if let Some(tok) = parser.curr() {
        if tok.is_unquoted() && matches!(tok.as_str().to_lowercase().as_str(), "stored" | "virtual") {
            parser.next();

            if let Some(tok) = parser.curr() {
                if tok.is_unquoted() && tok.as_str().to_lowercase() == "generated" {
                    parser.next();

                    extra.generated = true;
                }
            }
        }
    }

    extra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            parse_column_extra(&mut Parser::new("")),
            ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: false,
            }
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            parse_column_extra(&mut Parser::new("DEFAULT_GENERATED")),
            ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: true,
            }
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            parse_column_extra(&mut Parser::new("DEFAULT_GENERATED on update CURRENT_TIMESTAMP")),
            ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: true,
                generated: false,
                default_generated: true,
            }
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            parse_column_type(into_tokens("smallint unsigned")),
            ColumnType::SmallInt(NumericAttr {
                maximum: None,
                decimal: None,
                unsigned: Some(true),
                zero_fill: None,
            })
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            parse_column_type(into_tokens("smallint unsigned zerofill")),
            ColumnType::SmallInt(NumericAttr {
                maximum: None,
                decimal: None,
                unsigned: Some(true),
                zero_fill: Some(true),
            })
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            parse_column_type(into_tokens("decimal(4,2)")),
            ColumnType::Decimal(NumericAttr {
                maximum: Some(4),
                decimal: Some(2),
                unsigned: None,
                zero_fill: None,
            })
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            parse_column_type(into_tokens("decimal(18,4) zerofill")),
            ColumnType::Decimal(NumericAttr {
                maximum: Some(18),
                decimal: Some(4),
                unsigned: None,
                zero_fill: Some(true),
            })
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            parse_column_type(into_tokens("decimal(18,4) unsigned")),
            ColumnType::Decimal(NumericAttr {
                maximum: Some(18),
                decimal: Some(4),
                unsigned: Some(true),
                zero_fill: None,
            })
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            parse_column_type(into_tokens("decimal(18,4) unsigned zerofill")),
            ColumnType::Decimal(NumericAttr {
                maximum: Some(18),
                decimal: Some(4),
                unsigned: Some(true),
                zero_fill: Some(true),
            })
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            parse_column_type(into_tokens("smallint(8) unsigned zerofill")),
            ColumnType::SmallInt(NumericAttr {
                maximum: Some(8),
                decimal: None,
                unsigned: Some(true),
                zero_fill: Some(true),
            })
        );
    }

    #[test]
    fn test_10() {
        assert_eq!(
            parse_column_type(into_tokens("DATETIME")),
            ColumnType::DateTime(TimeAttr {
                fractional: None,
            })
        );
    }

    #[test]
    fn test_11() {
        assert_eq!(
            parse_column_type(into_tokens("DATETIME(6)")),
            ColumnType::DateTime(TimeAttr {
                fractional: Some(6),
            })
        );
    }

    #[test]
    fn test_12() {
        assert_eq!(
            parse_column_type(into_tokens("TIMESTAMP(0)")),
            ColumnType::Timestamp(TimeAttr {
                fractional: Some(0),
            })
        );
    }

    #[test]
    fn test_13() {
        assert_eq!(
            parse_column_type(into_tokens("varchar(20)")),
            ColumnType::Varchar(StringAttr {
                length: Some(20),
                charset_name: None,
                collation_name: None,
            })
        );
    }

    #[test]
    fn test_14() {
        assert_eq!(
            parse_column_type(into_tokens("TEXT")),
            ColumnType::Text(StringAttr {
                length: None,
                charset_name: None,
                collation_name: None,
            })
        );
    }

    #[test]
    fn test_15() {
        assert_eq!(
            parse_column_type(into_tokens("TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_bin")),
            ColumnType::Text(StringAttr {
                length: None,
                charset_name: Some("utf8mb4".to_owned()),
                collation_name: Some("utf8mb4_bin".to_owned()),
            })
        );
    }

    #[test]
    fn test_16() {
        assert_eq!(
            parse_column_type(into_tokens("TEXT CHARACTER SET latin1")),
            ColumnType::Text(StringAttr {
                length: None,
                charset_name: Some("latin1".to_owned()),
                collation_name: None,
            })
        );
    }

    #[test]
    fn test_17() {
        assert_eq!(
            parse_column_type(into_tokens("BLOB")),
            ColumnType::Blob(BlobAttr {
                length: None,
            })
        );
    }

    #[test]
    fn test_18() {
        assert_eq!(
            parse_column_type(into_tokens("BLOB(256)")),
            ColumnType::Blob(BlobAttr {
                length: Some(256),
            })
        );
    }

    #[test]
    fn test_19() {
        assert_eq!(
            parse_column_type(into_tokens("enum('G','PG','PG-13','R','NC-17')")),
            ColumnType::Enum(EnumDef {
                values: vec![
                    "G".to_owned(),
                    "PG".to_owned(),
                    "PG-13".to_owned(),
                    "R".to_owned(),
                    "NC-17".to_owned(),
                ],
                attr: StringAttr {
                    length: None,
                    charset_name: None,
                    collation_name: None,
                }
            })
        );
    }

    #[test]
    fn test_20() {
        assert_eq!(
            parse_column_type(into_tokens("set('Trailers','Commentaries','Deleted Scenes','Behind the Scenes')")),
            ColumnType::Set(SetDef {
                members: vec![
                    "Trailers".to_owned(),
                    "Commentaries".to_owned(),
                    "Deleted Scenes".to_owned(),
                    "Behind the Scenes".to_owned(),
                ],
                attr: StringAttr {
                    length: None,
                    charset_name: None,
                    collation_name: None,
                }
            })
        );
    }

    #[test]
    fn test_21() {
        assert_eq!(
            parse_column_type(into_tokens("GEOMETRY")),
            ColumnType::Geometry(GeometryAttr {
                srid: None,
            })
        );
    }

    #[test]
    fn test_22() {
        assert_eq!(
            parse_column_type(into_tokens("GEOMETRY SRID 4326")),
            ColumnType::Geometry(GeometryAttr {
                srid: Some(4326),
            })
        );
    }
}