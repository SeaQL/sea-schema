//! To parse mysql's index schema

use crate::mysql::def::*;
use crate::mysql::query::IndexQueryResult;

pub struct IndexQueryResultParser {
    curr: Option<IndexInfo>,
    results: Box<dyn Iterator<Item = IndexQueryResult>>,
}

/// IndexQueryResult must be sorted by (TableName, IndexName, SeqInIndex)
pub fn parse_index_query_results(results: Box<dyn Iterator<Item = IndexQueryResult>>) -> impl Iterator<Item = IndexInfo> {
    IndexQueryResultParser {
        curr: None,
        results,
    }
}

impl Iterator for IndexQueryResultParser {
    type Item = IndexInfo;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.results.next() {
            let mut index = parse_index_query_result(result);
            if let Some(curr) = &mut self.curr {
                if curr.name == index.name {
                    curr.columns.push(index.columns.pop().unwrap());
                } else {
                    let prev = self.curr.take();
                    self.curr = Some(index);
                    return prev;
                }
            } else {
                self.curr = Some(index);
            }
        }
        self.curr.take()
    }
}

pub fn parse_index_query_result(mut result: IndexQueryResult) -> IndexInfo {
    IndexInfo {
        unique: match result.non_unique {
            0 => true,
            1 => false,
            _ => unimplemented!(),
        },
        name: result.index_name,
        columns: vec![
            if result.column_name.is_some() {
                result.column_name.take().unwrap()
            } else if result.expression.is_some() {
                result.expression.take().unwrap()
            } else {
                panic!("index column error")
            }
        ],
        order: match result.collation {
            Some(collation) => match collation.as_str() {
                "A" => IndexOrder::Ascending,
                "D" => IndexOrder::Descending,
                _ => unimplemented!(),
            },
            None => IndexOrder::Unordered,
        },
        sub_part: match result.sub_part {
            Some(v) => Some(v as u32),
            None => None,
        },
        nullable: match result.nullable.as_str() {
            "YES" => true,
            _ => false,
        },
        idx_type: match result.index_type.as_str() {
            "BTREE" => IndexType::BTree,
            "FULLTEXT" => IndexType::FullText,
            "HASH" => IndexType::Hash,
            "RTREE" => IndexType::RTree,
            "SPATIAL" => IndexType::Spatial,
            _ => unimplemented!(),
        },
        comment: result.index_comment,
        functional: result.expression.is_some(),
    }
}