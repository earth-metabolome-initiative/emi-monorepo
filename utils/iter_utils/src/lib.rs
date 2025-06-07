#![doc = include_str!("../README.md")]

pub mod intersection;

/// A module that provides a trait for iterators that can be intersected.
pub mod prelude {
    pub use super::intersection::{Intersection, IntersectionIter};
}
