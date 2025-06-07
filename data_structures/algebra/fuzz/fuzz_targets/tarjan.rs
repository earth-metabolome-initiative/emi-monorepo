//! Submodule for fuzzing the execution of the Hopcroft-Karp algorithm.

use algebra::prelude::{CSR2D, SquareCSR2D, Tarjan};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: SquareCSR2D<CSR2D<u16, u8, u8>>| {
            let _ = csr.tarjan().collect::<Vec<_>>();
        });
    }
}
