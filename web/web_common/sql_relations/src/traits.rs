//! Submodule providing trait definitions describing abstractions over
//! PostgreSQL relations.

mod authored;
pub use authored::{AuthoredDatabaseLike, AuthoredTableLike};
mod most_concrete;
pub use most_concrete::{InheritableDatabaseLike, MostConcreteTableLike};
mod same_as;
pub use same_as::{
    HorizontalSameAsForeignKeyLike, HorizontalSameAsTableLike, SameAsIndexLike, SameAsTableLike,
    VerticalSameAsForeignKeyLike, VerticalSameAsTableLike,
};
