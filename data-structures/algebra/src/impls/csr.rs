//! Submodule providing a definition of a CSR matrix.

pub mod csr2d;
pub use csr2d::CSR2D;
pub mod upper_triangular_csr2d;
pub use upper_triangular_csr2d::UpperTriangularCSR2D;
pub mod square_csr2d;
pub use square_csr2d::SquareCSR2D;
pub mod csr2d_view;
pub use csr2d_view::CSR2DView;
pub mod csr2d_rows;
pub use csr2d_rows::CSR2DRows;
pub mod error;
pub use error::*;
pub mod symmetric_csr2d;
pub use symmetric_csr2d::SymmetricCSR2D;