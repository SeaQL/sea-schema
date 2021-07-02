mod column;
mod constraints;
mod table;

pub use column::*;
pub use constraints::*;
pub use table::*;

fn yes_or_no_to_bool(string: &str) -> bool {
    matches!(string.to_uppercase().as_str(), "YES")
}
