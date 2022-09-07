use sea_query::{Alias, IntoColumnRef, SimpleExpr};

pub mod char_set;
pub mod column;
pub mod constraints;
pub mod enumeration;
pub mod schema;
pub mod table;

pub use char_set::*;
pub use column::*;
pub use constraints::*;
pub use enumeration::*;
pub use schema::*;
pub use table::*;

pub(crate) fn cast_cols_as_int8<T, I>(cols: I) -> Vec<SimpleExpr>
where
    T: IntoColumnRef,
    I: IntoIterator<Item = T>,
{
    cols.into_iter()
        .map(|col| SimpleExpr::Column(col.into_column_ref()).cast_as(Alias::new("INT8")))
        .collect()
}
