//! Library containing core structures for the web application.
pub mod codegen;
pub use codegen::*;
pub mod document;
pub mod impls;
pub mod traits;

#[cfg(feature = "postgres")]
pub use document::create_photograph;
