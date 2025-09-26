
use algebra::prelude::{SquareCSR2D, CSR2D};
use graph::prelude::{GenericGraph, InformationContent};
use honggfuzz::fuzz;
use graph::traits::MonopartiteGraph;
use functional_properties::similarity::ScalarSimilarity;

fn main() {
    loop {
        fuzz!(|occurrences_csr: (Vec<f64>, GenericGraph<u8, SquareCSR2D<CSR2D<u16, u8, u8>>>)| {
            let (occurrences, csr) = occurrences_csr;
            let Ok(information_content) = csr.information_content(occurrences.as_ref()) else {
                return;
            };
           todo()
           //need to figure out fuzzer implementation
        });
    }
}