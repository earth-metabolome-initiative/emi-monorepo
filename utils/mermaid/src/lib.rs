#![doc = include_str!("../README.md")]
pub mod diagrams;
mod errors;
mod shared;
pub mod traits;
pub use errors::{
    ClassDiagramConfigError, ClassDiagramEdgeError, ClassDiagramError, ClassDiagramNodeError,
    ERDiagramConfigError, ERDiagramEdgeError, ERDiagramError, ERDiagramNodeError,
    FlowchartConfigError, FlowchartEdgeError, FlowchartError, FlowchartNodeError,
};

/// Submodule providing common traits and types for Mermaid diagrams.
pub mod prelude {
    pub use crate::{
        diagrams::{class_diagram::*, entity_relationship::*, flowchart::*},
        shared::{
            ArrowShape, Color, Direction, LineStyle, Renderer, StyleClass, StyleClassBuilder,
            StyleProperty, Unit,
        },
        traits::*,
    };
}
