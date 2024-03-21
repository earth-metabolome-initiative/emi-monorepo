//! Submodule providing structs to validate Images, provided as a vector of bytes.

pub mod image;
pub mod contains_face;
pub mod squarish;
pub use image::*;
pub use contains_face::*;
pub use squarish::*;

pub mod nudity;
pub use nudity::*;