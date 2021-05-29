#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod mysql;
pub mod name;
pub(crate) mod parser;
pub mod postgresql;
pub(crate) mod sqlx_types;
pub(crate) mod util;

pub use name::*;
