mod column;
mod pg_indexes;
mod table;
mod table_constraints;

pub use column::*;
pub use pg_indexes::*;
pub use table::*;
pub use table_constraints::*;

fn yes_or_no_to_bool(string: &str) -> bool {
    matches!(string.to_uppercase().as_str(), "YES")
}
