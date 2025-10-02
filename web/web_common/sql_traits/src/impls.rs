//! Submodule providing implementations of the traits defined in the `traits`
//! module.

mod sqlparser;
#[cfg(feature = "sqlparser")]
pub use sqlparser::SqlParserDatabase;