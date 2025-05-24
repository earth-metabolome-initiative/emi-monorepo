//! Submodule benchmarking the computation of the oxidation states of the
//! `MolecularFormula` struct.

use std::{hint::black_box, str::FromStr};

use criterion::Criterion;
use molecular_formulas::MolecularFormula;

/// Benchmark for the `oxidation_states` method.
fn bench_oxidation_states(c: &mut Criterion) {
    let mut epimeloscine_group = c.benchmark_group("large_molecular_formulas");
    epimeloscine_group.sample_size(10);

    let formula = MolecularFormula::from_str("2(C17H23NO3).H2O.H2SO4").unwrap();

    epimeloscine_group.bench_function("2(C17H23NO3).H2O.H2SO4", |b| {
        b.iter(|| formula.oxidation_states());
    });

    epimeloscine_group.bench_function("valid_0", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(0)));
    });

    epimeloscine_group.bench_function("valid_1", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(1)));
    });
    epimeloscine_group.bench_function("valid_-1", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(-1)));
    });
    epimeloscine_group.bench_function("valid_2", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(2)));
    });
    epimeloscine_group.bench_function("valid_-2", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(-2)));
    });
    epimeloscine_group.bench_function("valid_3", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(3)));
    });
    epimeloscine_group.bench_function("valid_-3", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(-3)));
    });
    epimeloscine_group.bench_function("valid_4", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(4)));
    });
    epimeloscine_group.bench_function("valid_-4", |b| {
        b.iter(|| formula.is_valid_oxidation_state(black_box(-4)));
    });

    epimeloscine_group.finish();
}

/// Function to run all benchmarks.
pub fn benches() {
    let mut criterion: ::criterion::Criterion<_> =
        ::criterion::Criterion::default().configure_from_args();
    bench_oxidation_states(&mut criterion);
}

/// Main function to run the benchmarks.
fn main() {
    benches();
    ::criterion::Criterion::default().configure_from_args().final_summary();
}
