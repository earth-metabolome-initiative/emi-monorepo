//! Submodule for fuzzing the execution of the Hopcroft-Karp algorithm.

use algebra::prelude::{HopcroftKarp, CSR2D};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: CSR2D<u16, u8, u8>| {
            let _ = csr.hopcroft_karp();
        });
    }
}
