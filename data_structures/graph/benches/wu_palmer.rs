//! Criterion benchmark to evaluate the performance of the 'wu-palmer' function.

use std::hint::black_box;

use algebra::impls::{CSR2D, SquareCSR2D};
use criterion::{Criterion, criterion_group, criterion_main};
use functional_properties::similarity::ScalarSimilarity;
use graph::{
    prelude::{GenericGraph, RandomizedDAG},
    traits::{MonopartiteGraph, WuPalmer, randomized_graphs::XorShift64},
};

/// Benchmark for the `wu-palmer` function
fn bench_wu_palmer(c: &mut Criterion) {
    c.bench_function("wu_palmer_10", |b| {
        const NUMBER_OF_DAGS: usize = 10;
        let mut dags = Vec::with_capacity(NUMBER_OF_DAGS);
        let mut xorshift = XorShift64::from(24537839457);
        for _ in 0..NUMBER_OF_DAGS {
            let seed = xorshift.next().unwrap();
            let dag: GenericGraph<u64, SquareCSR2D<CSR2D<u64, u64, u64>>> =
                RandomizedDAG::randomized_dag(seed, 10);
            dags.push(dag);
        }
        b.iter(|| {
            let mut total_similarity = 0.0;
            for dag in &dags {
                let wu_palmer = dag.wu_palmer().unwrap();

                for src in black_box(dag.node_ids()) {
                    for dst in black_box(dag.node_ids()) {
                        total_similarity += wu_palmer.similarity(black_box(&src), black_box(&dst));
                    }
                }
            }
            total_similarity
        })
    });
}

criterion_group!(benches, bench_wu_palmer);
criterion_main!(benches);
