mod column;
mod constraints;

pub use column::*;
pub use constraints::*;

fn yes_or_no_to_bool(string: &str) -> bool {
    matches!(string.to_uppercase().as_str(), "YES")
}
