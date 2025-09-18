//! Submodule for fuzzing the execution of the RootNodes algorithm.

use algebra::prelude::{SquareCSR2D, CSR2D};
use graph::prelude::{GenericGraph, RootNodes};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: GenericGraph<u8, SquareCSR2D<CSR2D<u16, u8, u8>>>| {
            let _root_nodes = csr.root_nodes();
        });
    }
}
