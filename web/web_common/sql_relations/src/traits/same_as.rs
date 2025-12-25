//! Submodule providing traits relative to the `same-as` relationships.

mod same_as_index;
pub use same_as_index::SameAsIndexLike;
mod same_as_table;
pub use same_as_table::SameAsTableLike;
mod vertical_same_as;
pub use vertical_same_as::{
    VerticalSameAsColumnLike, VerticalSameAsForeignKeyLike, VerticalSameAsTableLike,
};
mod horizontal_same_as;
pub use horizontal_same_as::{
    HorizontalSameAsColumnLike, HorizontalSameAsForeignKeyLike, HorizontalSameAsTableLike,
};
mod triangular_same_as;
pub use triangular_same_as::{
    TriangularSameAsColumnLike, TriangularSameAsForeignKeyLike, TriangularSameAsTableLike,
};
