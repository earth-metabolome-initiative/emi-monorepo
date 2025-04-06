//! Submodule providing implementations of the traits.

pub mod csr;
pub mod error;
pub mod generic_bimatrix2d;
pub mod generic_implicit_valued_matrix2d;
pub mod generic_iterators;
pub mod generic_matrix2d_with_padded_diagonal;
pub mod ranged_csr;
pub mod valued_matrix;
pub mod padded_matrix2d;
pub mod vec;
mod vector;

pub use csr::*;
pub use error::{Error, MutabilityError};
pub use generic_bimatrix2d::GenericBiMatrix2D;
pub use generic_implicit_valued_matrix2d::GenericImplicitValuedMatrix2D;
pub use generic_iterators::*;
pub use generic_matrix2d_with_padded_diagonal::GenericMatrix2DWithPaddedDiagonal;
pub use ranged_csr::*;
pub use valued_matrix::*;
pub use padded_matrix2d::*;
pub use vec::VecMatrix2D;
