//! Submodule providing crates that define algorithms for graphs.

pub mod connected_components;
pub use connected_components::ConnectedComponents;
pub mod cycle_detection;
pub use cycle_detection::CycleDetection;
pub mod root_nodes;
pub use root_nodes::RootNodes;
pub mod sink_nodes;
pub use sink_nodes::SinkNodes;
