//! Submodule providing structs used across different type of Mermaid diagrams.

pub mod generic_configuration;
pub use generic_configuration::{Direction, Renderer};
pub(crate) mod generic_diagram;
pub mod javascript_function_signature;
pub mod javascript_types;
pub mod style_class;
pub use style_class::{
    Color, FontWeight, StyleClass, StyleClassBuilder, StyleClassError, StyleProperty, Unit,
};
pub mod arrow_shape;
pub mod click_event;
pub mod line_style;
pub use arrow_shape::ArrowShape;
pub use click_event::ClickEvent;
pub use line_style::LineStyle;
mod constants;
pub(crate) use constants::{EDGE_LETTER, NODE_LETTER};
pub(crate) mod generic_node;
pub(crate) use generic_node::GenericNode;
pub(crate) mod generic_edge;
pub(crate) use generic_edge::GenericEdge;
