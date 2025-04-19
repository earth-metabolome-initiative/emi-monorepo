//! Submodule providing algorithms for algebraic structures.

mod assignment;
pub use assignment::*;
mod weighted_assignment;
pub use weighted_assignment::*;
mod kahn;
pub use kahn::*;
mod johnson;
pub use johnson::*;
mod tarjan;
pub use tarjan::*;