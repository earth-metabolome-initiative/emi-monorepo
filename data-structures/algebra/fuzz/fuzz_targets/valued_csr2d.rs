//! Fuzzing submodule on the `ValuedCSR2D` struct.

use algebra::prelude::*;
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: ValuedCSR2D<u16, u8, u8, f64>| {
            // We check that the length of values returned for the values
            // and for the column indices is the same.
            for row_index in csr.row_indices() {
                let column_indices = csr.sparse_row(row_index).collect::<Vec<_>>();
                let column_values = csr.sparse_row_values(row_index).collect::<Vec<_>>();
                assert_eq!(
                    column_indices.len(),
                    column_values.len(),
                    "The row {row_index} has different lengths for column indices and values"
                );
            }
        });
    }
}
