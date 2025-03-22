//! Submodule providing implementations of the traits.

pub mod csr;
pub mod ranged_csr;
pub mod generic_bimatrix2d;
pub mod generic_implicit_valued_matrix2d;
pub mod vec;
mod vector;
pub mod generic_iterators;
pub mod error;
pub mod valued_matrix;
pub mod valued_matrix_columns;

pub use csr::*;
pub use ranged_csr::*;
pub use generic_bimatrix2d::GenericBiMatrix2D;
pub use generic_implicit_valued_matrix2d::GenericImplicitValuedMatrix2D;
pub use vec::VecMatrix2D;
pub use generic_iterators::*;
pub use error::{Error, MutabilityError};
pub use valued_matrix::*;
pub use valued_matrix_columns::*;