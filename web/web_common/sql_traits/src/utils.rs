//! Submodule providing utilities for SQL traits.

mod normalize_postgres_type;
pub use normalize_postgres_type::normalize_postgres_type;
mod normalize_sqlparser_type;
pub use normalize_sqlparser_type::normalize_sqlparser_type;
