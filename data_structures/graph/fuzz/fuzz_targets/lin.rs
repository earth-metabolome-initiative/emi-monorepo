use algebra::prelude::{SquareCSR2D, CSR2D};
use graph::prelude::{GenericGraph, Lin};
use honggfuzz::fuzz;
use graph::traits::MonopartiteGraph;
use functional_properties::similarity::ScalarSimilarity;

fn main() {
    loop {
        fuzz!(|occurrences_csr: (Vec<usize>,GenericGraph<u8, SquareCSR2D<CSR2D<u16, u8, u8>>>)| {
            let (occurrences, csr) = occurrences_csr;
            let Ok(lin) = csr.lin(occurrences.as_ref()) else {
                return ;
            };
            for src in csr.node_ids().take(10) {
                for dst in csr.node_ids(){
                    let similarity = lin.similarity(&src, &dst);
                    if src == dst {
                        assert!(similarity > 0.99);
                    } else {
                        let symmetric_similarity = lin.similarity(&dst, &src);
                        assert!({symmetric_similarity - similarity}.abs() < f64::EPSILON, "Expected sim({src}, {dst}) == sim({dst},{src}) got {similarity}!={symmetric_similarity}, with occurrences: {occurrences:?}");
                    }
                    assert!(similarity <= 1.0, "Expected sim({src},{dst}) = {similarity} <= 1");
                    assert!(similarity >= 0.0, "Expected sim({src},{dst}) = {similarity} >= 0");
                }
            }
        });
    }
}
