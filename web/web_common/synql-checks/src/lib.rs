#![doc = include_str!("../README.md")]

pub mod check_constraint;

/// Prelude module re-exporting commonly used traits and types.
pub mod prelude {
    pub use crate::check_constraint::CheckConstraintSynLike;
}
