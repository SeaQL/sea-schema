#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// Information relating to the table as a whole
pub struct TableInfo {
    pub name: String,
    pub comment: String,
}
