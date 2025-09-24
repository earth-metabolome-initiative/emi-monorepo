//! Submodule defining functions which are employed throughout the
//! `pg_relations` crate to analyze and abstract over PostgreSQL relations.

mod same_as_index;
pub(crate) use same_as_index::is_same_as_index;
