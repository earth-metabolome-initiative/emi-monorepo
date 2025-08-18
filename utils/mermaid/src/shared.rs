//! Submodule providing structs used across different type of Mermaid diagrams.

pub mod configuration;
pub mod diagram_properties;
pub mod duration;
pub(crate) mod generic_diagram;
pub mod javascript_function_signature;
pub mod javascript_types;
pub mod label;
pub mod style_class;
pub use style_class::{StyleClass, StyleClassBuilder};
