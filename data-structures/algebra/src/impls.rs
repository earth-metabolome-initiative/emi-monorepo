//! Submodule providing implementations of the traits.

pub mod csr;
pub mod vec;
pub mod bimatrix2d;
pub use csr::*;
pub use vec::VecMatrix2D;
pub use bimatrix2d::BiMatrix2D;