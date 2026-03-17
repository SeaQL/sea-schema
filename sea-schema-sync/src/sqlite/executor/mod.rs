#![allow(clippy::non_minimal_cfg)]

#[cfg(feature = "rusqlite")]
mod rusqlite;
#[cfg(feature = "rusqlite")]
pub use rusqlite::*;

#[cfg(all(feature = "sqlx-sqlite"))]
mod real;
#[cfg(all(feature = "sqlx-sqlite"))]
pub use real::*;

#[cfg(all(
    not(feature = "rusqlite"),
    all(not(feature = "rusqlite"), not(feature = "sqlx-sqlite"))
))]
mod mock;
#[cfg(all(
    not(feature = "rusqlite"),
    all(not(feature = "rusqlite"), not(feature = "sqlx-sqlite"))
))]
pub use mock::*;
