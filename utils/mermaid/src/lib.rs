#![doc = include_str!("../README.md")]
pub mod diagrams;
mod errors;
mod shared;
pub mod traits;
pub use errors::{ConfigError, EdgeError, NodeError, StyleClassError};

/// Submodule providing common traits and types for Mermaid diagrams.
pub mod prelude {
    pub use crate::{
        diagrams::{class_diagram::*, entity_relationship::*, flowchart::*},
        shared::{
            ArrowShape, Color, Direction, FontWeight, LineStyle, Renderer, StyleClass,
            StyleClassBuilder, StyleProperty, Unit,
        },
        traits::*,
    };
}
