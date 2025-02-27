//! Submodule providing implementations of the traits.

pub mod csr;
pub mod generic_bimatrix2d;
pub mod vec;
pub use csr::*;
pub use generic_bimatrix2d::GenericBiMatrix2D;
pub use vec::VecMatrix2D;
