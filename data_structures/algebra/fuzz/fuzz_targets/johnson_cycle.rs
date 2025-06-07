//! Submodule for fuzzing the execution of the Hopcroft-Karp algorithm.

use algebra::prelude::{CSR2D, Johnson, Matrix2D, SparseMatrix, SquareCSR2D};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: SquareCSR2D<CSR2D<u16, u8, u8>>| {
            let _ = csr.johnson().collect::<Vec<_>>();
        });
    }
}
