//! Benchmark module to compare the times necessary to answer to a trigram similarity
//! query on species names using either Diesel + PostgreSQL or alternatively Ngrammatics.
#![feature(test)]
extern crate test;
use backend::models::Taxa;
use criterion::{criterion_group, criterion_main, Criterion};
use diesel::{r2d2::ConnectionManager, PgConnection};
use ngrammatic::prelude::*;
use rayon::slice::ParallelSliceMut;
use std::fmt::Debug;

/// Returns an iterator over the taxons in the corpus.
fn iter_taxons() -> impl Iterator<Item = String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open("./db_data/taxons.tsv").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().split('\t').nth(1).unwrap().to_string())
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
        .set_minimum_similarity_score(0.6)
        .unwrap()
        // The old approach by default returned 10 results, so
        // to better compare the two, we set the same limit here.
        .set_maximum_number_of_results(10);

    c.bench_function(
        &format!(
            "ngram_search_{}_{}_{}",
            // We print the name of the Graph data type used
            std::any::type_name::<G>(),
            std::any::type_name::<R>(),
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

fn postgres_diesel(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager: ConnectionManager<_> = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> = match r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager)
    {
        Ok(client) => {
            log::info!("✅ Diesel connection to the database is successful!");
            client
        }
        Err(e) => {
            log::error!("🔥 Error connecting to the database with Diesel: {}", e);
            std::process::exit(1);
        }
    };

    let mut connection = pool.get().unwrap();

    c.bench_function("postgres_diesel", |b| {
        b.iter(|| {
            let _ = Taxa::search("Acanthocephala", Some(10), Some(0.6), &mut connection);
            let _ = Taxa::search("Doggus Lionenus", Some(10), Some(0.6), &mut connection);
            let _ = Taxa::search("Felis Caninus", Some(10), Some(0.6), &mut connection);
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(100));
    targets = ngrammatic_webgraph_trigram_search, ngrammatic_bitvec_trigram_search, postgres_diesel, ngrammatic_webgraph_trigram_par_search, ngrammatic_bitvec_trigram_par_search, ngrammatic_webgraph_trigram_search_lowercased, ngrammatic_bitvec_trigram_search_lowercased, ngrammatic_webgraph_trigram_par_search_lowercased, ngrammatic_bitvec_trigram_par_search_lowercased
);

criterion_main!(benches);
