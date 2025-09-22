use algebra::prelude::{SquareCSR2D, CSR2D};
use graph::prelude::{GenericGraph, Resnik};
use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|csr: GenericGraph<u8, SquareCSR2D<CSR2D<u16, u8, u8>>>| {
            let resnik = csr.resnik();
            for src in csr.nodes() {
                for dst in csr.nodes() {
                    resnik.similarity(&src, &dst);
                }
            }
        });
    }
}
