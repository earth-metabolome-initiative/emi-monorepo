//! Submodule defining structures used in the guided procedure builder.

mod hierarchy;
pub(crate) use hierarchy::Hierarchy;
mod task_graph;
pub(crate) use task_graph::TaskGraph;
mod ownership;
pub use hierarchy::HierarchyLike;
pub(crate) use ownership::Ownership;
pub use ownership::OwnershipLike;
