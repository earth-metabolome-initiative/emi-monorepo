//! Submodule providing crates that define algorithms for graphs.

pub mod connected_components;
pub use connected_components::ConnectedComponents;
pub use weighted_assignment::{HungarianAlgorithm, WeightedAssignment};
