use algebra::prelude::{SquareCSR2D, CSR2D};
use graph::prelude::{GenericGraph, Resnik};
use honggfuzz::fuzz;
use graph::traits::MonopartiteGraph;
use functional_properties::similarity::ScalarSimilarity;

fn main() {
    loop {
        fuzz!(|occurrences_csr: (Vec<f64>,GenericGraph<u8, SquareCSR2D<CSR2D<u16, u8, u8>>>)| {
            let (occurrences, csr) = occurrences_csr;
            let Ok(resnik) = csr.resnik(occurrences.as_ref()) else {
                return ;
            };
            for src in csr.node_ids().take(10) {
                for dst in csr.node_ids(){
                    resnik.similarity(&src, &dst);
                }
            }
        });
    }
}
