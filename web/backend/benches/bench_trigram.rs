//! Benchmark module to compare the times necessary to answer to a trigram similarity
//! query on species names using either Diesel + PostgreSQL or alternatively Ngrammatics.
#![feature(test)]
extern crate test;
use backend::models::BioOttTaxonItem;
use criterion::{criterion_group, criterion_main, Criterion};
use diesel::{r2d2::ConnectionManager, PgConnection};
use ngrammatic::prelude::*;
use rayon::slice::ParallelSliceMut;
use std::fmt::Debug;

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./db_data/bio_ott_taxons.csv.gz").unwrap();
    let reader = BufReader::new(flate2::read::GzDecoder::new(file));
    reader
        .lines()
        .map(|line| line.unwrap().split(',').nth(1).unwrap().to_string())
}

fn build_vec() -> Vec<String> {
    iter_taxons().collect()
}

/// Returns the built RCL
fn build_rcl() -> RearCodedList {
    let mut taxons: Vec<String> = build_vec();
    taxons.par_sort_unstable();

    let mut rcl_builder = RearCodedListBuilder::new(8);
    for taxon in taxons {
        rcl_builder.push(&taxon);
    }
    rcl_builder.build()
}
/// Returns ngram par-corpus.
fn new_corpus_bitvec<NG, B, R>(keys: B) -> Corpus<B, NG, R>
where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    R: ngrammatic::Key<NG, ngrammatic::ASCIIChar> + ?Sized,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<R>,
{
    Corpus::par_from(keys)
}

/// Returns ngram webgraph-based par-corpus.
fn new_corpus_webgraph<NG, B, R>(keys: B) -> Corpus<B, NG, R, BiWebgraph>
where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    R: ngrammatic::Key<NG, ngrammatic::ASCIIChar> + ?Sized,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<R>,
{
    Corpus::try_from(new_corpus_bitvec::<NG, B, R>(keys)).unwrap()
}

fn ngram_search<NG, B, G, R>(
    c: &mut Criterion,
    corpus: Corpus<B, NG, R, G>,
    search: fn(&Corpus<B, NG, R, G>, &str, NgramSearchConfig),
    parallel: bool,
) where
    B: Keys<NG>,
    NG: Ngram<G = ASCIIChar> + Debug,
    G: WeightedBipartiteGraph,
    R: ngrammatic::Key<NG, ngrammatic::ASCIIChar> + ?Sized,
    for<'a> <B as ngrammatic::Keys<NG>>::KeyRef<'a>: AsRef<R>,
{
    let search_config = NgramSearchConfig::default()
        .set_minimum_similarity_score(0.3)
        .unwrap()
        // The old approach by default returned 10 results, so
        // to better compare the two, we set the same limit here.
        .set_maximum_number_of_results(10);

    c.bench_function(
        &format!(
            "{}_{}_{}",
            // We print the name of the Graph data type used
            std::any::type_name::<G>().split("::").last().unwrap(),
            std::any::type_name::<R>().split("::").last().unwrap(),
            if parallel { "par" } else { "seq" }
        ),
        |b| {
            b.iter(|| {
                // Then we measure the time it takes to recreate
                // the corpus from scratch several times.
                search(&corpus, "Acanthocephala", search_config);
                search(&corpus, "Doggus Lionenus", search_config);
                search(&corpus, "Felis Caninus", search_config);
            });
        },
    );
}

fn ngrammatic_webgraph_trigram_search(c: &mut Criterion) {
    let corpus = new_corpus_webgraph::<TriGram<ASCIIChar>, RearCodedList, str>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_search(key, search_config);
        },
        false,
    );
}

fn ngrammatic_bitvec_trigram_search(c: &mut Criterion) {
    let corpus = new_corpus_bitvec::<TriGram<ASCIIChar>, RearCodedList, str>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_search(key, search_config);
        },
        false,
    );
}

fn ngrammatic_webgraph_trigram_par_search(c: &mut Criterion) {
    let corpus = new_corpus_webgraph::<TriGram<ASCIIChar>, RearCodedList, str>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_par_search(key, search_config);
        },
        true,
    );
}

fn ngrammatic_bitvec_trigram_par_search(c: &mut Criterion) {
    let corpus = new_corpus_bitvec::<TriGram<ASCIIChar>, RearCodedList, str>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_par_search(key, search_config);
        },
        true,
    );
}

fn ngrammatic_webgraph_trigram_search_lowercased(c: &mut Criterion) {
    let corpus =
        new_corpus_webgraph::<TriGram<ASCIIChar>, RearCodedList, Lowercase<str>>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_search(key, search_config);
        },
        false,
    );
}

fn ngrammatic_bitvec_trigram_search_lowercased(c: &mut Criterion) {
    let corpus =
        new_corpus_bitvec::<TriGram<ASCIIChar>, RearCodedList, Lowercase<str>>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_search(key, search_config);
        },
        false,
    );
}

fn ngrammatic_webgraph_trigram_par_search_lowercased(c: &mut Criterion) {
    let corpus =
        new_corpus_webgraph::<TriGram<ASCIIChar>, RearCodedList, Lowercase<str>>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_par_search(key, search_config);
        },
        true,
    );
}

fn ngrammatic_bitvec_trigram_par_search_lowercased(c: &mut Criterion) {
    let corpus =
        new_corpus_bitvec::<TriGram<ASCIIChar>, RearCodedList, Lowercase<str>>(build_rcl());
    ngram_search(
        c,
        corpus,
        |corpus, key, search_config| {
            corpus.ngram_par_search(key, search_config);
        },
        true,
    );
}

fn postgres_similarity(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager: ConnectionManager<_> = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager).unwrap();

    let mut connection = pool.get().unwrap();

    c.bench_function("postgres_similarity", |b| {
        b.iter(|| {
            let _ = BioOttTaxonItem::similarity_search("Acanthocephala", Some(10), &mut connection);
            let _ = BioOttTaxonItem::similarity_search("Doggus Lionenus", Some(10), &mut connection);
            let _ = BioOttTaxonItem::similarity_search("Felis Caninus", Some(10), &mut connection);
        });
    });
}

fn postgres_word_similarity(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager: ConnectionManager<_> = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager).unwrap();

    let mut connection = pool.get().unwrap();

    c.bench_function("postgres_word_similarity", |b| {
        b.iter(|| {
            let _ = BioOttTaxonItem::word_similarity_search("Acanthocephala", Some(10), &mut connection);
            let _ = BioOttTaxonItem::word_similarity_search("Doggus Lionenus", Some(10), &mut connection);
            let _ = BioOttTaxonItem::word_similarity_search("Felis Caninus", Some(10), &mut connection);
        });
    });
}

fn postgres_strict_word_similarity(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager: ConnectionManager<_> = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager).unwrap();

    let mut connection = pool.get().unwrap();

    c.bench_function("postgres_strict_word_similarity", |b| {
        b.iter(|| {
            let _ = BioOttTaxonItem::strict_word_similarity_search("Acanthocephala", Some(10), &mut connection);
            let _ = BioOttTaxonItem::strict_word_similarity_search("Doggus Lionenus", Some(10), &mut connection);
            let _ = BioOttTaxonItem::strict_word_similarity_search("Felis Caninus", Some(10), &mut connection);
        });
    });
}

criterion_group!(
    name = ngrammatic_benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(100));
    targets = ngrammatic_webgraph_trigram_search, ngrammatic_bitvec_trigram_search, ngrammatic_webgraph_trigram_par_search, ngrammatic_bitvec_trigram_par_search, ngrammatic_webgraph_trigram_search_lowercased, ngrammatic_bitvec_trigram_search_lowercased, ngrammatic_webgraph_trigram_par_search_lowercased, ngrammatic_bitvec_trigram_par_search_lowercased
);

criterion_group!(
    name = postgres_benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(100));
    targets = postgres_similarity, postgres_word_similarity, postgres_strict_word_similarity
);

criterion_main!(postgres_benches);
