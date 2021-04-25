use std::rc::Rc;
use sea_query::{Alias, Iden, Index, IndexCreateStatement};
use crate::mysql::def::{IndexInfo, IndexOrder, IndexType};

impl IndexInfo {
    pub fn write(&self) -> IndexCreateStatement {
        let mut index = Index::create();
        if self.name == "PRIMARY" {
            index = index.primary();
        } else {
            index = index.name(&self.name);
            if self.unique {
                index = index.unique();
            }
        }
        for part in self.parts.iter() {
            index = index.col(Alias::new(&part.column));
            if part.sub_part.is_some() {
                todo!();
            }
            if self.parts.len() == 1 {
                match part.order {
                    IndexOrder::Ascending => {},
                    IndexOrder::Descending => todo!(),
                    IndexOrder::Unordered => {},
                }
            }
        }
        match self.idx_type {
            IndexType::BTree => {},
            IndexType::FullText => { index = index.index_type(sea_query::IndexType::FullText) },
            IndexType::Hash => { index = index.index_type(sea_query::IndexType::Hash) },
            IndexType::RTree => { index = index.index_type(sea_query::IndexType::Custom(Rc::new(Alias::new(&self.idx_type.to_string())))) },
            IndexType::Spatial => { index = index.index_type(sea_query::IndexType::Custom(Rc::new(Alias::new(&self.idx_type.to_string())))) },
        }
        index
    }
}
