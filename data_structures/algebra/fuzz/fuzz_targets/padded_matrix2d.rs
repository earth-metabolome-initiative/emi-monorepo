//! Fuzzing submodule on the `PaddedMatrix2d` struct.

use algebra::prelude::*;
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: ValuedCSR2D<u16, u8, u8, u8>| {
            let padded_matrix = PaddedMatrix2D::new(&csr, |_| 1).unwrap();
			let padded_number_of_rows = padded_matrix.number_of_rows();
			let padded_number_of_columns = padded_matrix.number_of_columns();

			let csr_number_of_rows = csr.number_of_rows();
			let csr_number_of_columns = csr.number_of_columns();
			let mut last_tuple = None;

            // We check that all values in the csr matrix still appear
            // in the padded matrix.
            for row_index in csr.row_indices() {
                let csr_column_values = csr
                    .sparse_row(row_index)
                    .zip(csr.sparse_row_values(row_index))
                    .collect::<Vec<_>>();
                let padded_column_values = padded_matrix
                    .column_indices()
                    .zip(padded_matrix.row_values(row_index))
                    .collect::<Vec<_>>();

                for (column_index, value) in &csr_column_values {
                    assert!(
						padded_column_values.contains(&(*column_index, *value)),
						"The padded matrix does not contain the value {value} (last tuple was {last_tuple:?}) at column index {column_index}/{padded_number_of_columns} ({csr_number_of_columns}) for row index {row_index}/{padded_number_of_rows} ({csr_number_of_rows}). {csr:?}"
					);
					last_tuple = Some((*column_index, *value));
                }

				// We check that all values in the padded matrix are
				// either in the csr matrix or are imputed.
				for (column_index, value) in padded_column_values {
					if padded_matrix.is_imputed((row_index, column_index)) {
						assert_eq!(value, 1);
					} else {
						assert!(
							csr_column_values.contains(&(column_index, value)),
							"The csr matrix does not contain the value {value} at column index {column_index} for row index {row_index}"
						);
					}
				}
            }
        });
    }
}
