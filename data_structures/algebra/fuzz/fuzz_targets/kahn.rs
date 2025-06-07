//! Submodule for fuzzing the execution of the Kahn's algorithm.

use algebra::prelude::{
    CSR2D, IntoUsize, Kahn, Matrix2D, MatrixMut, SparseMatrix, SparseMatrix2D, SquareCSR2D,
    UpperTriangularCSR2D,
};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|matrix: SquareCSR2D<CSR2D<u16, u8, u8>>| {
            if matrix.number_of_rows() > 5 || matrix.number_of_columns() > 5 {
                return;
            }

            println!(
                "Fuzzing Kahn's algorithm with a {}x{} matrix, coordinates: {:?}",
                matrix.number_of_rows(),
                matrix.number_of_columns(),
                matrix.sparse_coordinates().collect::<Vec<_>>()
            );

            let kahn_ordering = matrix.kahn();

            if let Ok(ordering) = kahn_ordering {
                matrix.row_indices().for_each(|row_id| {
                    let resorted_row_id = ordering[row_id.into_usize()];
                    matrix.sparse_row(row_id).for_each(|successor_id| {
                        let resorted_successor_id = ordering[successor_id.into_usize()];
                        assert!(
                            resorted_row_id <= resorted_successor_id,
                            "The ordering {ordering:?} is not valid: {resorted_row_id} ({row_id}) > {resorted_successor_id} ({successor_id})",
                        );
                    });
                });

                // If the ordering is valid, it must be possible to constructed an
                // upper triangular matrix from the ordering.
                let mut coordinates = matrix
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
