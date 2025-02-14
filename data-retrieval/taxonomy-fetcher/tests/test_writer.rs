//! Test whether the  taxonomy writer works as expected.

use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use strum::IntoEnumIterator;
use taxonomy_fetcher::impls::ncbi::taxonomy_writer::NCBITaxonomyWriter;
use taxonomy_fetcher::impls::ncbi::NCBITaxonomyBuilder;
use taxonomy_fetcher::impls::ncbi::NCBIVersion;
use taxonomy_fetcher::Taxon;
use taxonomy_fetcher::TaxonomyBuilder;
use taxonomy_fetcher::Taxonomy;
use taxonomy_fetcher::TaxonomyWriter;

#[tokio::test]
async fn test_writer() {
    // We create a new OTOL taxonomy fetcher.
    let fetcher = NCBITaxonomyBuilder::latest();

    // We fetch the taxonomy.
    let taxonomy = fetcher.build().await.unwrap();
    
    NCBITaxonomyWriter::default()
    .ltree()
    .write(&taxonomy, "./NCBItaxonomy_ltree.tsv").unwrap();

    NCBITaxonomyWriter::default()
    .write(&taxonomy, "./NCBItaxonomy.tsv").unwrap();

}
