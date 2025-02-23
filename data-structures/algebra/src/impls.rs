//! Submodule providing implementations of the traits.

pub mod csr;
pub mod vec;
pub mod generic_bimatrix2d;
pub use csr::*;
pub use vec::VecMatrix2D;
pub use generic_bimatrix2d::GenericBiMatrix2D;