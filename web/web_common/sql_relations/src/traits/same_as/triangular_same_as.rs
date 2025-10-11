//! Submodule defining whether a relationship defined by a foreign key is a
//! triangular same-as relationship, i.e. defines a relationship from a table A
//! to a table B where A and B are already connected by some horizontal same-as
//! relationship, and A inherits from some ancestor C which is also referred to
//! by B. When the ancestor C identifier is the value linked by the horizontal
//! same-as relationship, then the relationship from A to B is a mandatory
//! triangular same-as relationship, otherwise it is a discretionary triangular
//! same-as relationship.

mod triangular_same_as_foreign_key;
pub use triangular_same_as_foreign_key::TriangularSameAsForeignKeyLike;
mod triangular_same_as_table;
pub use triangular_same_as_table::TriangularSameAsTableLike;
