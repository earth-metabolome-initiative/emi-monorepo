//! Submodule with utilities employed by the `pg_cached` crate.

mod function_hash;
mod is_conn_argument;
mod non_conn_arguments;
pub(crate) use function_hash::function_hash;
pub(crate) use non_conn_arguments::{first_non_conn_argument_ident, non_conn_arguments_idents};
