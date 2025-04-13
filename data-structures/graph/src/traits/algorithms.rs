//! Submodule providing crates that define algorithms for graphs.

pub mod connected_components;
pub use connected_components::ConnectedComponents;
pub mod topological_sorting;
pub use topological_sorting::TopologicalSorting;
pub mod cycle_detection;
pub use cycle_detection::CycleDetection;
pub mod root_nodes;
pub use root_nodes::RootNodes;
