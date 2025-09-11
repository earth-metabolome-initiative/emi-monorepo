//! Library containing core structures for the web application.
pub mod codegen;
pub use codegen::*;
pub mod document;
pub mod impls;
pub mod traits;

pub use document::create_photograph;
