//! Submodule providing implementations of the traits.

pub mod csr;
pub mod generic_bimatrix2d;
pub mod generic_implicit_valued_matrix2d;
pub mod vec;
mod vector;

pub use csr::*;
pub use generic_bimatrix2d::GenericBiMatrix2D;
pub use generic_implicit_valued_matrix2d::GenericImplicitValuedMatrix2D;
pub use vec::VecMatrix2D;
