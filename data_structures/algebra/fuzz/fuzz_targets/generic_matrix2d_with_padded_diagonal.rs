//! Fuzzing submodule on the `CSR2D` struct.

use algebra::prelude::*;
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|padded_csr: GenericMatrix2DWithPaddedDiagonal<
            ValuedCSR2D<u16, u8, u8, f64>,
            fn(u8) -> f64,
        >| {
            assert_eq!(
                padded_csr.number_of_rows(),
                padded_csr.number_of_columns(),
                "The number of rows and columns should be equal",
            );

            for row_index in padded_csr.row_indices() {
                // We check that the diagonal of the row is imputed.
                let mut sparse_column_indices = padded_csr.sparse_row(row_index);
                sparse_column_indices.find(|&column_index| column_index == row_index).expect(
                    "The diagonal of the row should always be imputed but was not found in the sparse row",
                );

                // We check that the number of sparse column indices and values are equal.
                let number_of_sparse_column_indices = padded_csr.sparse_row(row_index).count();
                let number_of_sparse_column_values =
                    padded_csr.sparse_row_values(row_index).count();

                assert_eq!(
                    number_of_sparse_column_indices, number_of_sparse_column_values,
                    "The number of sparse column indices and values should be equal"
                );

                // We check that the `is_diagonal_imputed` method works as expected
                // consistently.
                let underlying_matrix = padded_csr.matrix();
                let has_diagonal = if row_index < underlying_matrix.number_of_rows() {
                    underlying_matrix
                        .sparse_row(row_index)
                        .find(|&column_index| column_index == row_index)
                        .is_some()
                } else {
                    false
                };
                let is_diagonal_imputed = padded_csr.is_diagonal_imputed(row_index);
                assert_eq!(
                    has_diagonal, !is_diagonal_imputed,
                    "The inner diagonal state was `{has_diagonal}` but the `is_diagonal_imputed` method returned `{is_diagonal_imputed}`"
                );

                // We check that the number of elements in the internal sparse row
                // is equal to the number of elements in the padded sparse row, plus
                // the diagonal element if it has been imputed.
                let expected_number_of_elements = if row_index < underlying_matrix.number_of_rows()
                {
                    let number_of_inner_sparse_column_indices =
                        underlying_matrix.sparse_row(row_index).count();
                    if has_diagonal {
                        number_of_inner_sparse_column_indices
                    } else {
                        number_of_inner_sparse_column_indices + 1
                    }
                } else {
                    1
                };

                assert_eq!(
                    number_of_sparse_column_indices,
                    expected_number_of_elements,
                    "The number of elements in the padded sparse row should be equal to the number of elements in the inner sparse row plus the diagonal element if it has been imputed"
                );
            }
        });
    }
}
