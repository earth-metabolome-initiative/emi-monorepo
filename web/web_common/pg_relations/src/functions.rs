//! Submodule defining functions which are employed throughout the
//! `pg_relations` crate to analyze and abstract over PostgreSQL relations.

mod same_as_index;
pub(crate) use same_as_index::is_same_as_index;
mod extension_foreign_key;
pub(crate) use extension_foreign_key::is_extension_foreign_key;
mod ancestral_same_as_foreign_key;
mod associated_same_as_foreign_key;
mod partial_builder_foreign_key;
mod author_column;
mod most_concrete_column;