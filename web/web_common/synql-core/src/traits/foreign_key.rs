//! Submodule defining and implementing the `ForeignKeySynLike` trait, which
//! provides methods to facilitate the rust code generation starting from a SQL
//! foreign key representation, building on top of the
//! [`ForeignKeyLike`](sql_traits::traits::ForeignKeyLike) trait and the traits
//! from the [`sql_relations`](sql_relations) crate.

use sql_traits::traits::{ColumnLike, ForeignKeyLike};

/// Trait implemented by types that represent SQL foreign keys and can be used
/// to generate Rust code for them.
pub trait ForeignKeySynLike: ForeignKeyLike {
    /// Returns the name of the getter method associated with this constraint.
    fn foreign_key_getter_name(&self, database: &Self::DB) -> String {
        self.host_columns(database).map(|column| column.column_name()).collect::<Vec<_>>().join("_")
    }
}

impl<FK: ForeignKeyLike> ForeignKeySynLike for FK {}
