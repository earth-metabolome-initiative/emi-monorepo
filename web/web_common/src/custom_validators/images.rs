//! Submodule providing structs to validate Images, provided as a vector of bytes.

pub mod contains_face;
pub mod image;
pub mod squarish;
pub use contains_face::*;
pub use image::*;
pub use squarish::*;
pub mod range_shape;
pub use range_shape::*;
