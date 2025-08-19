//! Submodule providing structs used across different type of Mermaid diagrams.

pub mod configuration;
pub mod diagram_properties;
pub mod duration;
pub(crate) mod generic_diagram;
pub mod javascript_function_signature;
pub mod javascript_types;
pub mod style_class;
pub use style_class::{StyleClass, StyleClassBuilder, StyleClassError, StyleProperty};
pub mod arrow_shape;
pub mod line_style;
pub(crate) use arrow_shape::ArrowShape;
pub(crate) use line_style::LineStyle;
mod constants;
pub(crate) use constants::{EDGE_LETTER, NODE_LETTER};
pub(crate) mod generic_node;
pub(crate) use generic_node::GenericNode;
