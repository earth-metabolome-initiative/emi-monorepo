//! Submodule providing utilities for SQL traits.

mod normalize_postgres_type;
pub use normalize_postgres_type::normalize_postgres_type;
mod normalize_sqlparser_type;
pub use normalize_sqlparser_type::normalize_sqlparser_type;
mod columns_in_expression;
pub use columns_in_expression::columns_in_expression;
mod last_str;
pub use last_str::last_str;
