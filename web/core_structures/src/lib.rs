//! Library containing core structures for the web application.
pub mod codegen;
pub use codegen::*;
pub mod document;
mod procedure_models;

#[cfg(feature = "postgres")]
pub use document::create_photograph;
