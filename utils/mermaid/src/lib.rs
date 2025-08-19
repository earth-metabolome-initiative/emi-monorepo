#![doc = include_str!("../README.md")]
mod diagrams;
mod errors;
mod shared;
pub mod traits;
pub use diagrams::{ClassDiagram, EntityRelationshipDiagram, Flowchart};
