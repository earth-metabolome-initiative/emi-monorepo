use algebra::prelude::{SquareCSR2D, CSR2D};
use graph::prelude::{GenericGraph, Resnik};
use honggfuzz::fuzz;
use graph::traits::MonopartiteGraph;
use functional_properties::similarity::ScalarSimilarity;

fn main() {
    loop {
        fuzz!(|occurences_csr: (Option<Vec<usize>>,GenericGraph<u8, SquareCSR2D<CSR2D<u16, u8, u8>>>)| {
            let (occurences, csr) = occurences_csr;
            let Ok(resnik) = csr.resnik(occurences.as_deref()) else {
                return ;
            };
            for src in csr.node_ids() {
                for dst in csr.node_ids() {
                    resnik.similarity(&src, &dst);
                }
            }
        });
    }
}
