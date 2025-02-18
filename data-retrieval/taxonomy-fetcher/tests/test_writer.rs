//! Test whether the  taxonomy writer works as expected.

use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use std::time::Instant;
use strum::IntoEnumIterator;
use taxonomy_fetcher::impls::ncbi::taxonomy_writer::NCBITaxonomyWriter;
use taxonomy_fetcher::impls::ncbi::NCBITaxonomyBuilder;
use taxonomy_fetcher::impls::ncbi::NCBIVersion;
use taxonomy_fetcher::Taxon;
use taxonomy_fetcher::Taxonomy;
use taxonomy_fetcher::TaxonomyBuilder;
use taxonomy_fetcher::TaxonomyWriter;
use std::path::Path;

#[tokio::test]
async fn test_writer() {
    // Logger is initialized
    env_logger::init();

    // We create a new OTOL taxonomy fetcher.
    let fetcher = NCBITaxonomyBuilder::latest();

    // We fetch the taxonomy.
    log::info!("Now fetching taxonomy from fetcher");
    let start = Instant::now();
    let taxonomy = fetcher.build().await.unwrap();
    let duration = start.elapsed();
    log::info!("Taxonomy fetching took: {:?}", duration);

    log::info!("Outputting the standard compressed taxonomy.");
    let start = Instant::now();
    NCBITaxonomyWriter::default()
        .compressed()
        .write(&taxonomy, Path::new("./NCBItaxonomy.tsv.gz"))
        .unwrap();
    let duration = start.elapsed();
    log::info!("Standard compressed taxonomy writing took: {:?}", duration);

    log::info!("Outputting the standard taxonomy.");
    let start = Instant::now();
    NCBITaxonomyWriter::default()
        .compressed()
        .write(&taxonomy, Path::new("./NCBItaxonomy.tsv"))
        .unwrap();
    let duration = start.elapsed();
    log::info!("Standard taxonomy writing took: {:?}", duration);

    log::info!("Outputting the ltree compressed taxonomy.");
    let start = Instant::now();
    NCBITaxonomyWriter::default()
        .ltree()
        .compressed()
        .write(&taxonomy, Path::new("./NCBItaxonomy_ltree.tsv.gz"))
        .unwrap();
    let duration = start.elapsed();
    log::info!("Ltree compressed taxonomy writing took: {:?}", duration);

    log::info!("Outputting the ltree taxonomy.");
    let start = Instant::now();
    NCBITaxonomyWriter::default()
        .ltree()
        .write(&taxonomy, Path::new("./NCBItaxonomy_ltree.tsv"))
        .unwrap();
    let duration = start.elapsed();
    log::info!("Ltree taxonomy writing took: {:?}", duration);
}
