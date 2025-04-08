//! Fuzzing submodule on the `CSR2D` struct.

use algebra::prelude::*;
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: CSR2D<u16, u8, u8>| {
            // We collect the output of all rows and check that they are sorted.
            for row_index in csr.row_indices() {
                let column_indices = csr.sparse_row(row_index).collect::<Vec<_>>();
                let mut sorted_column_indices = column_indices.clone();
                sorted_column_indices.sort_unstable();
                assert_eq!(
                    column_indices, sorted_column_indices,
                    "The row {row_index} is not sorted"
                );
                sorted_column_indices.dedup();
                assert_eq!(
                    column_indices, sorted_column_indices,
                    "The row {row_index} has duplicates"
                );
            }

            // We collect the sparse coordinates, and check that they are sorted
            // and non-duplicated.
            let sparse_coordinates = csr.sparse_coordinates().collect::<Vec<_>>();
            let mut clone_sparse_coordinates = sparse_coordinates.clone();
            clone_sparse_coordinates.sort_unstable();
            assert_eq!(
                sparse_coordinates, clone_sparse_coordinates,
                "The sparse coordinates are not sorted"
            );
            clone_sparse_coordinates.dedup();
            assert_eq!(
                sparse_coordinates, clone_sparse_coordinates,
                "The sparse coordinates have duplicates: {:?}",
                csr
            );
        });
    }
}
