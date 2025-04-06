//! Submodule for fuzzing the execution of the `SparseLAPMOD` algorithm.

use algebra::prelude::{HopcroftKarp, SparseLAPJV, Matrix2D, SparseValuedMatrix, ValuedCSR2D};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: ValuedCSR2D<u16, u8, u8, f64>| {
            let maximum_value = csr.max_sparse_value().unwrap_or(1000.0);
            let padding_value = (maximum_value + 1.0) * 2.0;
            let maximal_cost = (padding_value + 1.0) * 2.0;

            // Due to the inexact nature of floating point arithmetic, we need to ensure that
            // the padding value is greater than the maximum value in the matrix and that the
            // maximal cost is greater than the padding value.
            if maximum_value >= padding_value {
                return;
            }

            if maximal_cost <= padding_value {
                return;
            }

            let Ok(lapmod_assignment) = csr.sparse_lapjv(padding_value, maximal_cost) else {
                return;
            };
            let Ok(hopcroft_karp_assignment) = csr.hopcroft_karp() else {
                return;
            };
            assert_eq!(
                lapmod_assignment.len(),
                hopcroft_karp_assignment.len(),
                "LAPMOD and Hopcroft-Karp assignments have different lengths: {:?}",
                csr
            );
        });
    }
}
