//! Submodule for fuzzing the execution of the Kahn's algorithm.

use algebra::prelude::{
    CSR2D, IntoUsize, Kahn, MatrixMut, SparseMatrix, SquareCSR2D, UpperTriangularCSR2D,
};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: SquareCSR2D<CSR2D<u16, u8, u8>>| {
            let kahn_ordering = csr.kahn();
            if let Ok(ordering) = kahn_ordering {
                // If the ordering is valid, it must be possible to constructed an
                // upper triangular matrix from the ordering.
                let mut coordinates = csr
                    .sparse_coordinates()
                    .map(|(i, j)| (ordering[i.into_usize()], ordering[j.into_usize()]))
                    .collect::<Vec<_>>();
                coordinates.sort_unstable();

                let _triangular: UpperTriangularCSR2D<CSR2D<u16, u8, u8>> =
                    UpperTriangularCSR2D::from_entries(coordinates)
                        .expect("The ordering should be valid");
            }
        });
    }
}
