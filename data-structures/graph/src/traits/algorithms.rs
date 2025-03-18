//! Submodule providing crates that define algorithms for graphs.

pub mod connected_components;
pub mod assignment;

pub use connected_components::ConnectedComponents;
pub use assignment::{HungarianAlgorithm, Assignment};